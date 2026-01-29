use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::app::{App, Mode};
use crate::store::Store;

pub fn handle_key_event(app: &mut App, key: KeyEvent, store: &Store) {
    match app.mode {
        Mode::Normal => handle_normal_mode(app, key),
        Mode::Add | Mode::Edit => handle_form_mode(app, key, store),
        Mode::Delete => handle_delete_mode(app, key, store),
        Mode::Help => handle_help_mode(app, key),
    }
}

fn handle_normal_mode(app: &mut App, key: KeyEvent) {
    match (key.code, key.modifiers) {
        // Quit
        (KeyCode::Esc, _) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
            app.cancel();
        }

        // Select
        (KeyCode::Enter, _) => {
            app.select_current();
        }

        // Navigation
        (KeyCode::Up, _) | (KeyCode::Char('k'), KeyModifiers::CONTROL) => {
            app.move_up();
        }
        (KeyCode::Down, _) | (KeyCode::Char('j'), KeyModifiers::CONTROL) => {
            app.move_down();
        }
        (KeyCode::Tab, KeyModifiers::NONE) => {
            app.move_down();
        }
        (KeyCode::BackTab, _) => {
            app.move_up();
        }

        // Actions (Function keys to avoid conflicts)
        (KeyCode::F(2), _) => {
            app.enter_add_mode();
        }
        (KeyCode::F(3), _) => {
            app.enter_edit_mode();
        }
        (KeyCode::F(4), _) => {
            app.enter_delete_mode();
        }
        (KeyCode::F(1), _) => {
            app.toggle_help();
        }

        // Cursor movement
        (KeyCode::Left, _) | (KeyCode::Char('b'), KeyModifiers::CONTROL) => {
            app.move_cursor_left();
        }
        (KeyCode::Right, _) | (KeyCode::Char('f'), KeyModifiers::CONTROL) => {
            app.move_cursor_right();
        }
        (KeyCode::Home, _) | (KeyCode::Char('a'), KeyModifiers::CONTROL) => {
            app.move_cursor_start();
        }
        (KeyCode::End, _) | (KeyCode::Char('e'), KeyModifiers::CONTROL) => {
            app.move_cursor_end();
        }

        // Deletion
        (KeyCode::Backspace, _) | (KeyCode::Char('h'), KeyModifiers::CONTROL) => {
            app.delete_char();
        }
        (KeyCode::Delete, _) | (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
            app.delete_char_forward();
        }
        (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
            app.clear_input();
        }

        // Typing
        (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) => {
            app.insert_char(c);
        }

        _ => {}
    }
}

fn handle_form_mode(app: &mut App, key: KeyEvent, store: &Store) {
    match (key.code, key.modifiers) {
        (KeyCode::Esc, _) => {
            app.cancel();
        }
        (KeyCode::Enter, _) => {
            app.confirm_action(store);
        }
        (KeyCode::Tab, KeyModifiers::NONE) => {
            app.next_field();
        }
        (KeyCode::BackTab, _) => {
            app.prev_field();
        }
        (KeyCode::Backspace, _) => {
            app.delete_char();
        }
        (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
            app.clear_input();
        }
        (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) => {
            app.insert_char(c);
        }
        _ => {}
    }
}

fn handle_delete_mode(app: &mut App, key: KeyEvent, store: &Store) {
    match key.code {
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            app.confirm_action(store);
        }
        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
            app.cancel();
        }
        _ => {}
    }
}

fn handle_help_mode(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('?') | KeyCode::Char('q') => {
            app.toggle_help();
        }
        _ => {}
    }
}
