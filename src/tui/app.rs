use crate::command::Command;
use crate::config::Config;
use crate::error::{CmdxError, Result};
use crate::store::Store;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

use super::event::handle_key_event;
use super::ui::draw_ui;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Normal,
    Add,
    Edit,
    Delete,
    Help,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputField {
    Path,
    Command,
    Description,
}

pub struct App {
    pub input: String,
    pub cursor_position: usize,
    pub commands: Vec<Command>,
    pub filtered: Vec<(usize, i64)>,
    pub selected: usize,
    pub scroll_offset: usize,
    pub visible_height: usize,
    pub should_quit: bool,
    pub selected_command: Option<Command>,
    pub mode: Mode,
    pub form_path: String,
    pub form_command: String,
    pub form_description: String,
    pub active_field: InputField,
    pub message: Option<(String, bool)>, // (message, is_error)
    pub editing_original_path: Option<String>,
    matcher: SkimMatcherV2,
}

impl App {
    pub fn new(commands: Vec<Command>) -> Self {
        let len = commands.len();
        let filtered: Vec<(usize, i64)> = (0..len).map(|i| (i, 0)).collect();

        Self {
            input: String::new(),
            cursor_position: 0,
            commands,
            filtered,
            selected: 0,
            scroll_offset: 0,
            visible_height: 10,
            should_quit: false,
            selected_command: None,
            mode: Mode::Normal,
            form_path: String::new(),
            form_command: String::new(),
            form_description: String::new(),
            active_field: InputField::Path,
            message: None,
            editing_original_path: None,
            matcher: SkimMatcherV2::default(),
        }
    }

    pub fn update_filter(&mut self) {
        if self.input.is_empty() {
            self.filtered = (0..self.commands.len()).map(|i| (i, 0)).collect();
        } else {
            let query = &self.input;
            let mut scored: Vec<(usize, i64)> = self
                .commands
                .iter()
                .enumerate()
                .filter_map(|(idx, cmd)| {
                    let path_score = self.matcher.fuzzy_match(&cmd.path, query);
                    let cmd_score = self.matcher.fuzzy_match(&cmd.command, query);
                    let explanation_score = self.matcher.fuzzy_match(&cmd.explanation, query);

                    let best_score = [path_score, cmd_score, explanation_score]
                        .into_iter()
                        .flatten()
                        .max();

                    best_score.map(|score| (idx, score))
                })
                .collect();

            scored.sort_by(|a, b| b.1.cmp(&a.1));
            self.filtered = scored;
        }

        if self.selected >= self.filtered.len() {
            self.selected = 0;
        }
        self.scroll_offset = 0;
    }

    fn ensure_visible(&mut self) {
        if self.selected < self.scroll_offset {
            self.scroll_offset = self.selected;
        } else if self.selected >= self.scroll_offset + self.visible_height {
            self.scroll_offset = self.selected.saturating_sub(self.visible_height - 1);
        }
    }

    pub fn move_up(&mut self) {
        if !self.filtered.is_empty() && self.selected > 0 {
            self.selected -= 1;
            self.ensure_visible();
        }
    }

    pub fn move_down(&mut self) {
        if !self.filtered.is_empty() && self.selected < self.filtered.len() - 1 {
            self.selected += 1;
            self.ensure_visible();
        }
    }

    pub fn set_visible_height(&mut self, height: usize) {
        self.visible_height = height.max(1);
        self.ensure_visible();
    }

    pub fn select_current(&mut self) {
        if let Some(&(idx, _)) = self.filtered.get(self.selected) {
            self.selected_command = Some(self.commands[idx].clone());
        }
        self.should_quit = true;
    }

    pub fn cancel(&mut self) {
        match self.mode {
            Mode::Normal => self.should_quit = true,
            _ => {
                self.mode = Mode::Normal;
                self.clear_form();
                self.message = None;
            }
        }
    }

    pub fn insert_char(&mut self, c: char) {
        match self.mode {
            Mode::Normal => {
                self.input.insert(self.cursor_position, c);
                self.cursor_position += 1;
                self.update_filter();
            }
            Mode::Add | Mode::Edit => {
                let field = self.get_active_field_mut();
                field.push(c);
            }
            _ => {}
        }
    }

    pub fn delete_char(&mut self) {
        match self.mode {
            Mode::Normal => {
                if self.cursor_position > 0 {
                    self.cursor_position -= 1;
                    self.input.remove(self.cursor_position);
                    self.update_filter();
                }
            }
            Mode::Add | Mode::Edit => {
                let field = self.get_active_field_mut();
                field.pop();
            }
            _ => {}
        }
    }

    pub fn delete_char_forward(&mut self) {
        if self.mode == Mode::Normal && self.cursor_position < self.input.len() {
            self.input.remove(self.cursor_position);
            self.update_filter();
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.input.len() {
            self.cursor_position += 1;
        }
    }

    pub fn move_cursor_start(&mut self) {
        self.cursor_position = 0;
    }

    pub fn move_cursor_end(&mut self) {
        self.cursor_position = self.input.len();
    }

    pub fn clear_input(&mut self) {
        match self.mode {
            Mode::Normal => {
                self.input.clear();
                self.cursor_position = 0;
                self.update_filter();
            }
            Mode::Add | Mode::Edit => {
                let field = self.get_active_field_mut();
                field.clear();
            }
            _ => {}
        }
    }

    pub fn visible_range(&self) -> std::ops::Range<usize> {
        let start = self.scroll_offset;
        let end = (self.scroll_offset + self.visible_height).min(self.filtered.len());
        start..end
    }

    pub fn enter_add_mode(&mut self) {
        self.mode = Mode::Add;
        self.clear_form();
        self.active_field = InputField::Path;
        self.message = None;
    }

    pub fn enter_edit_mode(&mut self) {
        if let Some(&(idx, _)) = self.filtered.get(self.selected) {
            let cmd = &self.commands[idx];
            self.form_path = cmd.path.clone();
            self.form_command = cmd.command.clone();
            self.form_description = cmd.explanation.clone();
            self.editing_original_path = Some(cmd.path.clone());
            self.mode = Mode::Edit;
            self.active_field = InputField::Command;
            self.message = None;
        }
    }

    pub fn enter_delete_mode(&mut self) {
        if !self.filtered.is_empty() {
            self.mode = Mode::Delete;
            self.message = None;
        }
    }

    pub fn toggle_help(&mut self) {
        self.mode = if self.mode == Mode::Help {
            Mode::Normal
        } else {
            Mode::Help
        };
    }

    pub fn next_field(&mut self) {
        self.active_field = match self.active_field {
            InputField::Path => InputField::Command,
            InputField::Command => InputField::Description,
            InputField::Description => InputField::Path,
        };
    }

    pub fn prev_field(&mut self) {
        self.active_field = match self.active_field {
            InputField::Path => InputField::Description,
            InputField::Command => InputField::Path,
            InputField::Description => InputField::Command,
        };
    }

    fn get_active_field_mut(&mut self) -> &mut String {
        match self.active_field {
            InputField::Path => &mut self.form_path,
            InputField::Command => &mut self.form_command,
            InputField::Description => &mut self.form_description,
        }
    }

    fn clear_form(&mut self) {
        self.form_path.clear();
        self.form_command.clear();
        self.form_description.clear();
        self.editing_original_path = None;
    }

    pub fn confirm_action(&mut self, store: &Store) {
        match self.mode {
            Mode::Add => self.save_new_command(store),
            Mode::Edit => self.save_edited_command(store),
            Mode::Delete => self.delete_selected_command(store),
            _ => {}
        }
    }

    fn save_new_command(&mut self, store: &Store) {
        if self.form_path.is_empty() || self.form_command.is_empty() {
            self.message = Some(("Path and command are required".to_string(), true));
            return;
        }

        let cmd = Command::new(&self.form_path, &self.form_command, &self.form_description);
        match store.add(&cmd, false) {
            Ok(()) => {
                self.commands.push(cmd);
                self.update_filter();
                self.mode = Mode::Normal;
                self.clear_form();
                self.message = Some(("Command added successfully".to_string(), false));
            }
            Err(e) => {
                self.message = Some((format!("Error: {}", e), true));
            }
        }
    }

    fn save_edited_command(&mut self, store: &Store) {
        if self.form_path.is_empty() || self.form_command.is_empty() {
            self.message = Some(("Path and command are required".to_string(), true));
            return;
        }

        let original_path = self.editing_original_path.as_ref().unwrap().clone();
        
        // Remove old command
        if let Err(e) = store.remove(&original_path) {
            self.message = Some((format!("Error: {}", e), true));
            return;
        }

        // Add updated command
        let cmd = Command::new(&self.form_path, &self.form_command, &self.form_description);
        match store.add(&cmd, false) {
            Ok(()) => {
                // Update in-memory list
                if let Some(idx) = self.commands.iter().position(|c| c.path == original_path) {
                    self.commands[idx] = cmd;
                }
                self.update_filter();
                self.mode = Mode::Normal;
                self.clear_form();
                self.message = Some(("Command updated successfully".to_string(), false));
            }
            Err(e) => {
                // Try to restore old command on failure
                let _ = store.add(
                    &Command::new(&original_path, &self.form_command, &self.form_description),
                    true,
                );
                self.message = Some((format!("Error: {}", e), true));
            }
        }
    }

    fn delete_selected_command(&mut self, store: &Store) {
        if let Some(&(idx, _)) = self.filtered.get(self.selected) {
            let path = self.commands[idx].path.clone();
            match store.remove(&path) {
                Ok(()) => {
                    self.commands.remove(idx);
                    self.update_filter();
                    if self.selected >= self.filtered.len() && self.selected > 0 {
                        self.selected -= 1;
                    }
                    self.mode = Mode::Normal;
                    self.message = Some(("Command deleted".to_string(), false));
                }
                Err(e) => {
                    self.message = Some((format!("Error: {}", e), true));
                    self.mode = Mode::Normal;
                }
            }
        }
    }
}

pub fn run(commands: Vec<Command>) -> Result<Option<Command>> {
    let config = Config::load().unwrap_or_default();
    let store = Store::new(&config);

    enable_raw_mode().map_err(|e| CmdxError::Tui(e.to_string()))?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
        .map_err(|e| CmdxError::Tui(e.to_string()))?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).map_err(|e| CmdxError::Tui(e.to_string()))?;

    let mut app = App::new(commands);

    let result = loop {
        terminal
            .draw(|f| draw_ui(f, &mut app))
            .map_err(|e| CmdxError::Tui(e.to_string()))?;

        if let Event::Key(key) = event::read().map_err(|e| CmdxError::Tui(e.to_string()))? {
            handle_key_event(&mut app, key, &store);
        }

        if app.should_quit {
            break app.selected_command.clone();
        }
    };

    disable_raw_mode().map_err(|e| CmdxError::Tui(e.to_string()))?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .map_err(|e| CmdxError::Tui(e.to_string()))?;
    terminal
        .show_cursor()
        .map_err(|e| CmdxError::Tui(e.to_string()))?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_commands() -> Vec<Command> {
        vec![
            Command::new("git/status", "git status", "Show working tree status"),
            Command::new("git/commit", "git commit -m", "Commit changes"),
            Command::new("docker/ps", "docker ps -a", "List all containers"),
            Command::new("docker/prune", "docker system prune -af", "Remove unused data"),
        ]
    }

    #[test]
    fn test_app_new() {
        let commands = sample_commands();
        let app = App::new(commands.clone());

        assert_eq!(app.commands.len(), 4);
        assert_eq!(app.filtered.len(), 4);
        assert_eq!(app.selected, 0);
        assert_eq!(app.mode, Mode::Normal);
        assert!(app.input.is_empty());
        assert!(!app.should_quit);
    }

    #[test]
    fn test_filter_by_command() {
        let mut app = App::new(sample_commands());

        app.insert_char('g');
        app.insert_char('i');
        app.insert_char('t');

        assert_eq!(app.filtered.len(), 2);
    }

    #[test]
    fn test_filter_by_description() {
        let mut app = App::new(sample_commands());

        app.input = "container".to_string();
        app.cursor_position = 9;
        app.update_filter();

        assert_eq!(app.filtered.len(), 1);
        let (idx, _) = app.filtered[0];
        assert_eq!(app.commands[idx].path, "docker/ps");
    }

    #[test]
    fn test_filter_by_path() {
        let mut app = App::new(sample_commands());

        app.input = "docker".to_string();
        app.cursor_position = 6;
        app.update_filter();

        assert_eq!(app.filtered.len(), 2);
    }

    #[test]
    fn test_navigation_up_down() {
        let mut app = App::new(sample_commands());

        assert_eq!(app.selected, 0);

        app.move_down();
        assert_eq!(app.selected, 1);

        app.move_down();
        assert_eq!(app.selected, 2);

        app.move_up();
        assert_eq!(app.selected, 1);

        app.move_up();
        assert_eq!(app.selected, 0);

        // Should not go below 0
        app.move_up();
        assert_eq!(app.selected, 0);
    }

    #[test]
    fn test_navigation_bounds() {
        let mut app = App::new(sample_commands());

        // Move to end
        app.move_down();
        app.move_down();
        app.move_down();
        assert_eq!(app.selected, 3);

        // Should not exceed bounds
        app.move_down();
        assert_eq!(app.selected, 3);
    }

    #[test]
    fn test_select_current() {
        let mut app = App::new(sample_commands());

        app.move_down();
        app.select_current();

        assert!(app.should_quit);
        assert!(app.selected_command.is_some());
        assert_eq!(app.selected_command.unwrap().path, "git/commit");
    }

    #[test]
    fn test_cancel_normal_mode() {
        let mut app = App::new(sample_commands());

        app.cancel();

        assert!(app.should_quit);
        assert!(app.selected_command.is_none());
    }

    #[test]
    fn test_enter_add_mode() {
        let mut app = App::new(sample_commands());

        app.enter_add_mode();

        assert_eq!(app.mode, Mode::Add);
        assert!(app.form_path.is_empty());
        assert!(app.form_command.is_empty());
        assert!(app.form_description.is_empty());
        assert_eq!(app.active_field, InputField::Path);
    }

    #[test]
    fn test_enter_edit_mode() {
        let mut app = App::new(sample_commands());

        app.move_down(); // Select git/commit
        app.enter_edit_mode();

        assert_eq!(app.mode, Mode::Edit);
        assert_eq!(app.form_path, "git/commit");
        assert_eq!(app.form_command, "git commit -m");
        assert_eq!(app.form_description, "Commit changes");
        assert_eq!(app.active_field, InputField::Command);
    }

    #[test]
    fn test_enter_delete_mode() {
        let mut app = App::new(sample_commands());

        app.enter_delete_mode();

        assert_eq!(app.mode, Mode::Delete);
    }

    #[test]
    fn test_toggle_help() {
        let mut app = App::new(sample_commands());

        assert_eq!(app.mode, Mode::Normal);

        app.toggle_help();
        assert_eq!(app.mode, Mode::Help);

        app.toggle_help();
        assert_eq!(app.mode, Mode::Normal);
    }

    #[test]
    fn test_cancel_from_add_mode() {
        let mut app = App::new(sample_commands());

        app.enter_add_mode();
        app.form_path = "test/path".to_string();
        app.cancel();

        assert_eq!(app.mode, Mode::Normal);
        assert!(app.form_path.is_empty());
        assert!(!app.should_quit);
    }

    #[test]
    fn test_field_navigation() {
        let mut app = App::new(sample_commands());

        app.enter_add_mode();
        assert_eq!(app.active_field, InputField::Path);

        app.next_field();
        assert_eq!(app.active_field, InputField::Command);

        app.next_field();
        assert_eq!(app.active_field, InputField::Description);

        app.next_field();
        assert_eq!(app.active_field, InputField::Path);

        app.prev_field();
        assert_eq!(app.active_field, InputField::Description);
    }

    #[test]
    fn test_form_input() {
        let mut app = App::new(sample_commands());

        app.enter_add_mode();

        app.insert_char('t');
        app.insert_char('e');
        app.insert_char('s');
        app.insert_char('t');

        assert_eq!(app.form_path, "test");

        app.next_field();
        app.insert_char('c');
        app.insert_char('m');
        app.insert_char('d');

        assert_eq!(app.form_command, "cmd");
    }

    #[test]
    fn test_form_delete_char() {
        let mut app = App::new(sample_commands());

        app.enter_add_mode();
        app.form_path = "test".to_string();

        app.delete_char();
        assert_eq!(app.form_path, "tes");

        app.delete_char();
        assert_eq!(app.form_path, "te");
    }

    #[test]
    fn test_clear_input_normal_mode() {
        let mut app = App::new(sample_commands());

        app.input = "search".to_string();
        app.cursor_position = 6;

        app.clear_input();

        assert!(app.input.is_empty());
        assert_eq!(app.cursor_position, 0);
    }

    #[test]
    fn test_clear_input_form_mode() {
        let mut app = App::new(sample_commands());

        app.enter_add_mode();
        app.form_path = "some/path".to_string();

        app.clear_input();

        assert!(app.form_path.is_empty());
    }

    #[test]
    fn test_cursor_movement() {
        let mut app = App::new(sample_commands());

        app.input = "hello".to_string();
        app.cursor_position = 5;

        app.move_cursor_left();
        assert_eq!(app.cursor_position, 4);

        app.move_cursor_start();
        assert_eq!(app.cursor_position, 0);

        app.move_cursor_right();
        assert_eq!(app.cursor_position, 1);

        app.move_cursor_end();
        assert_eq!(app.cursor_position, 5);
    }

    #[test]
    fn test_visible_range() {
        let mut app = App::new(sample_commands());
        app.set_visible_height(2);

        let range = app.visible_range();
        assert_eq!(range, 0..2);

        app.move_down();
        app.move_down();
        let range = app.visible_range();
        assert_eq!(range.start, 1);
    }

    #[test]
    fn test_empty_commands() {
        let app = App::new(vec![]);

        assert!(app.commands.is_empty());
        assert!(app.filtered.is_empty());
        assert_eq!(app.selected, 0);
    }

    #[test]
    fn test_filter_no_match() {
        let mut app = App::new(sample_commands());

        app.input = "xyz123notexist".to_string();
        app.cursor_position = 14;
        app.update_filter();

        assert!(app.filtered.is_empty());
    }

    #[test]
    fn test_delete_char_forward() {
        let mut app = App::new(sample_commands());

        app.input = "hello".to_string();
        app.cursor_position = 2;

        app.delete_char_forward();
        assert_eq!(app.input, "helo");
    }

    #[test]
    fn test_selected_resets_on_filter() {
        let mut app = App::new(sample_commands());

        app.move_down();
        app.move_down();
        assert_eq!(app.selected, 2);

        app.input = "git".to_string();
        app.cursor_position = 3;
        app.update_filter();

        // Selected should reset when filter changes
        assert_eq!(app.selected, 0);
    }
}
