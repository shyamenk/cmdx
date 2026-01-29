use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, Clear, List, ListItem, Paragraph, Scrollbar,
        ScrollbarOrientation, ScrollbarState, Wrap,
    },
    Frame,
};

use super::app::{App, InputField, Mode};

#[allow(dead_code)]
mod cat {
    use ratatui::style::Color;

    pub const BASE: Color = Color::Rgb(30, 30, 46);
    pub const MANTLE: Color = Color::Rgb(24, 24, 37);
    pub const CRUST: Color = Color::Rgb(17, 17, 27);
    pub const TEXT: Color = Color::Rgb(205, 214, 244);
    pub const SUBTEXT1: Color = Color::Rgb(186, 194, 222);
    pub const SUBTEXT0: Color = Color::Rgb(166, 173, 200);
    pub const OVERLAY2: Color = Color::Rgb(147, 153, 178);
    pub const OVERLAY1: Color = Color::Rgb(127, 132, 156);
    pub const OVERLAY0: Color = Color::Rgb(108, 112, 134);
    pub const SURFACE2: Color = Color::Rgb(88, 91, 112);
    pub const SURFACE1: Color = Color::Rgb(69, 71, 90);
    pub const SURFACE0: Color = Color::Rgb(49, 50, 68);
    pub const LAVENDER: Color = Color::Rgb(180, 190, 254);
    pub const BLUE: Color = Color::Rgb(137, 180, 250);
    pub const SAPPHIRE: Color = Color::Rgb(116, 199, 236);
    pub const SKY: Color = Color::Rgb(137, 220, 235);
    pub const TEAL: Color = Color::Rgb(148, 226, 213);
    pub const GREEN: Color = Color::Rgb(166, 227, 161);
    pub const YELLOW: Color = Color::Rgb(249, 226, 175);
    pub const PEACH: Color = Color::Rgb(250, 179, 135);
    pub const MAROON: Color = Color::Rgb(235, 160, 172);
    pub const RED: Color = Color::Rgb(243, 139, 168);
    pub const MAUVE: Color = Color::Rgb(203, 166, 247);
    pub const PINK: Color = Color::Rgb(245, 194, 231);
    pub const FLAMINGO: Color = Color::Rgb(242, 205, 205);
    pub const ROSEWATER: Color = Color::Rgb(245, 224, 220);
}

pub fn draw_ui(f: &mut Frame, app: &mut App) {
    let size = f.area();

    // Full screen background
    let bg = Block::default().style(Style::default().bg(cat::BASE));
    f.render_widget(bg, size);

    // Main layout
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Search bar
            Constraint::Min(1),    // Content (list + preview)
        ])
        .split(size);

    draw_search_bar(f, app, main_chunks[0]);

    // Two columns: list and preview
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunks[1]);

    draw_command_list(f, app, columns[0]);
    draw_preview(f, app, columns[1]);

    // Modals
    match app.mode {
        Mode::Add | Mode::Edit => draw_form_modal(f, app, size),
        Mode::Delete => draw_delete_modal(f, app, size),
        Mode::Help => draw_help_modal(f, size),
        Mode::Normal => {}
    }
}

fn draw_search_bar(f: &mut Frame, app: &App, area: Rect) {
    let total = app.commands.len();
    let filtered = app.filtered.len();

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(1), Constraint::Length(15)])
        .split(area);

    // Prompt
    let cursor = if app.input.is_empty() { "│" } else { "" };
    let line = Line::from(vec![
        Span::styled("> ", Style::default().fg(cat::MAUVE)),
        Span::styled(&app.input, Style::default().fg(cat::TEXT)),
        Span::styled(cursor, Style::default().fg(cat::LAVENDER)),
    ]);
    f.render_widget(Paragraph::new(line), layout[0]);

    // Count
    let count = Line::from(vec![
        Span::styled(":", Style::default().fg(cat::OVERLAY0)),
        Span::styled(format!(" {}", filtered), Style::default().fg(cat::BLUE)),
        Span::styled("/", Style::default().fg(cat::OVERLAY0)),
        Span::styled(format!("{}", total), Style::default().fg(cat::OVERLAY1)),
    ]);
    f.render_widget(Paragraph::new(count).alignment(Alignment::Right), layout[1]);
}

fn draw_command_list(f: &mut Frame, app: &mut App, area: Rect) {
    // Title bar with dashes
    let title = format!("─ Commands ─");
    let title_line = Line::from(vec![
        Span::styled(title, Style::default().fg(cat::OVERLAY1)),
        Span::styled(
            "─".repeat(area.width.saturating_sub(12) as usize),
            Style::default().fg(cat::SURFACE1),
        ),
    ]);

    let block = Block::default()
        .borders(Borders::TOP)
        .border_style(Style::default().fg(cat::SURFACE1))
        .title(title_line)
        .style(Style::default().bg(cat::BASE));

    f.render_widget(block.clone(), area);
    let inner = Rect {
        x: area.x,
        y: area.y + 1,
        width: area.width,
        height: area.height.saturating_sub(1),
    };

    let visible_items = inner.height as usize;
    app.set_visible_height(visible_items.max(1));

    let visible_range = app.visible_range();

    let items: Vec<ListItem> = visible_range
        .clone()
        .map(|filtered_idx| {
            let (cmd_idx, _score) = app.filtered[filtered_idx];
            let cmd = &app.commands[cmd_idx];
            let is_selected = filtered_idx == app.selected;

            // Get icon and color based on category
            let (icon, icon_color) = get_category_icon(&cmd.path);
            let max_width = inner.width.saturating_sub(4) as usize;
            let path_display = truncate_str(&cmd.path, max_width);

            let line = if is_selected {
                Line::from(vec![
                    Span::styled(icon, Style::default().fg(icon_color)),
                    Span::styled(" ", Style::default()),
                    Span::styled(
                        path_display,
                        Style::default().fg(cat::TEXT).add_modifier(Modifier::BOLD),
                    ),
                ])
            } else {
                Line::from(vec![
                    Span::styled(icon, Style::default().fg(icon_color)),
                    Span::styled(" ", Style::default()),
                    Span::styled(path_display, Style::default().fg(cat::SUBTEXT0)),
                ])
            };

            if is_selected {
                ListItem::new(line).style(Style::default().bg(cat::SURFACE0))
            } else {
                ListItem::new(line)
            }
        })
        .collect();

    let list = List::new(items);
    f.render_widget(list, inner);

    // Scrollbar
    if app.filtered.len() > app.visible_height {
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None)
            .track_symbol(Some("│"))
            .thumb_symbol("█")
            .style(Style::default().fg(cat::SURFACE0))
            .thumb_style(Style::default().fg(cat::SURFACE2));

        let mut scrollbar_state =
            ScrollbarState::new(app.filtered.len()).position(app.selected);

        f.render_stateful_widget(scrollbar, inner, &mut scrollbar_state);
    }
}

fn draw_preview(f: &mut Frame, app: &App, area: Rect) {
    // Get selected command for title
    let title_text = if let Some(&(idx, _)) = app.filtered.get(app.selected) {
        app.commands[idx].path.clone()
    } else {
        "Preview".to_string()
    };

    let title = format!("─ {} ─", title_text);
    let title_line = Line::from(vec![
        Span::styled(title, Style::default().fg(cat::OVERLAY1)),
        Span::styled(
            "─".repeat(area.width.saturating_sub(title_text.len() as u16 + 4) as usize),
            Style::default().fg(cat::SURFACE1),
        ),
    ]);

    let block = Block::default()
        .borders(Borders::TOP | Borders::LEFT)
        .border_style(Style::default().fg(cat::SURFACE1))
        .title(title_line)
        .style(Style::default().bg(cat::MANTLE));

    f.render_widget(block.clone(), area);

    let inner = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(1),
    };

    if let Some(&(idx, _)) = app.filtered.get(app.selected) {
        let cmd = &app.commands[idx];

        let mut lines: Vec<Line> = Vec::new();
        let line_num_width = 3;

        // Command with line numbers (syntax highlight style)
        let cmd_lines = wrap_text(&cmd.command, inner.width.saturating_sub(line_num_width + 2) as usize);
        for (i, line) in cmd_lines.iter().enumerate() {
            lines.push(Line::from(vec![
                Span::styled(
                    format!("{:>width$}  ", i + 1, width = line_num_width as usize),
                    Style::default().fg(cat::OVERLAY0),
                ),
                Span::styled(line, Style::default().fg(cat::TEXT)),
            ]));
        }

        // Empty line
        lines.push(Line::from(""));

        // Description with different color
        if !cmd.explanation.is_empty() {
            let desc_lines = wrap_text(&cmd.explanation, inner.width.saturating_sub(line_num_width + 2) as usize);
            for line in desc_lines.iter() {
                lines.push(Line::from(vec![
                    Span::styled(
                        format!("{:>width$}  ", "#", width = line_num_width as usize),
                        Style::default().fg(cat::OVERLAY0),
                    ),
                    Span::styled(line.clone(), Style::default().fg(cat::GREEN)),
                ]));
            }
        }

        // Help hints at bottom
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled("F1", Style::default().fg(cat::BLUE)),
            Span::styled(" help  ", Style::default().fg(cat::OVERLAY0)),
            Span::styled("F2", Style::default().fg(cat::GREEN)),
            Span::styled(" add  ", Style::default().fg(cat::OVERLAY0)),
            Span::styled("F3", Style::default().fg(cat::YELLOW)),
            Span::styled(" edit  ", Style::default().fg(cat::OVERLAY0)),
            Span::styled("F4", Style::default().fg(cat::RED)),
            Span::styled(" del", Style::default().fg(cat::OVERLAY0)),
        ]));

        f.render_widget(Paragraph::new(lines), inner);
    } else {
        let empty = Paragraph::new(Line::from(Span::styled(
            "No commands",
            Style::default().fg(cat::OVERLAY0),
        )));
        f.render_widget(empty, inner);
    }
}

fn get_category_icon(path: &str) -> (&'static str, ratatui::style::Color) {
    let category = path.split('/').next().unwrap_or("");
    match category {
        "git" => ("", cat::PEACH),
        "docker" => ("󰡨", cat::BLUE),
        "pg" | "postgres" | "db" => ("", cat::SAPPHIRE),
        "npm" | "node" => ("", cat::GREEN),
        "cargo" | "rust" => ("", cat::PEACH),
        "k8s" | "kubectl" => ("󱃾", cat::LAVENDER),
        "sys" | "linux" => ("", cat::YELLOW),
        "net" | "network" => ("󰛳", cat::TEAL),
        "ssh" => ("", cat::MAUVE),
        "dev" => ("", cat::PINK),
        "files" => ("", cat::ROSEWATER),
        _ => ("󰘧", cat::OVERLAY1),
    }
}

fn draw_form_modal(f: &mut Frame, app: &App, size: Rect) {
    let modal_area = centered_rect(50, 40, size);

    f.render_widget(Clear, modal_area);

    let title = if app.mode == Mode::Add {
        "─ New Command ─"
    } else {
        "─ Edit Command ─"
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(cat::LAVENDER))
        .title(Span::styled(title, Style::default().fg(cat::LAVENDER)))
        .style(Style::default().bg(cat::BASE));

    f.render_widget(block.clone(), modal_area);
    let inner = block.inner(modal_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(inner);

    draw_form_field(f, "path", &app.form_path, app.active_field == InputField::Path, chunks[0]);
    draw_form_field(f, "command", &app.form_command, app.active_field == InputField::Command, chunks[1]);
    draw_form_field(f, "description", &app.form_description, app.active_field == InputField::Description, chunks[2]);

    let hints = Line::from(vec![
        Span::styled("Tab", Style::default().fg(cat::OVERLAY1)),
        Span::styled(" next  ", Style::default().fg(cat::OVERLAY0)),
        Span::styled("Enter", Style::default().fg(cat::GREEN)),
        Span::styled(" save  ", Style::default().fg(cat::OVERLAY0)),
        Span::styled("Esc", Style::default().fg(cat::RED)),
        Span::styled(" cancel", Style::default().fg(cat::OVERLAY0)),
    ]);
    f.render_widget(Paragraph::new(hints).alignment(Alignment::Center), chunks[4]);
}

fn draw_form_field(f: &mut Frame, label: &str, value: &str, is_active: bool, area: Rect) {
    let label_color = if is_active { cat::LAVENDER } else { cat::OVERLAY0 };

    let label_area = Rect { height: 1, ..area };
    f.render_widget(
        Paragraph::new(Span::styled(label, Style::default().fg(label_color))),
        label_area,
    );

    let input_area = Rect { y: area.y + 1, height: 1, ..area };
    let display = if is_active {
        format!("{}│", value)
    } else if value.is_empty() {
        "─".to_string()
    } else {
        value.to_string()
    };

    let (fg, bg) = if is_active {
        (cat::TEXT, cat::SURFACE0)
    } else {
        (cat::SUBTEXT0, cat::MANTLE)
    };

    f.render_widget(
        Paragraph::new(display).style(Style::default().fg(fg).bg(bg)),
        input_area,
    );
}

fn draw_delete_modal(f: &mut Frame, app: &App, size: Rect) {
    let modal_area = centered_rect(40, 20, size);

    f.render_widget(Clear, modal_area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(cat::RED))
        .title(Span::styled("─ Delete? ─", Style::default().fg(cat::RED)))
        .style(Style::default().bg(cat::BASE));

    f.render_widget(block.clone(), modal_area);
    let inner = block.inner(modal_area);

    let cmd_name = if let Some(&(idx, _)) = app.filtered.get(app.selected) {
        truncate_str(&app.commands[idx].path, inner.width.saturating_sub(4) as usize)
    } else {
        String::new()
    };

    let content = vec![
        Line::from(""),
        Line::from(Span::styled(cmd_name, Style::default().fg(cat::YELLOW))),
        Line::from(""),
        Line::from(vec![
            Span::styled("y", Style::default().fg(cat::RED)),
            Span::styled(" yes  ", Style::default().fg(cat::OVERLAY0)),
            Span::styled("n", Style::default().fg(cat::OVERLAY1)),
            Span::styled(" no", Style::default().fg(cat::OVERLAY0)),
        ]),
    ];

    f.render_widget(
        Paragraph::new(content).alignment(Alignment::Center).wrap(Wrap { trim: true }),
        inner,
    );
}

fn draw_help_modal(f: &mut Frame, size: Rect) {
    let modal_area = centered_rect(45, 55, size);

    f.render_widget(Clear, modal_area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(cat::BLUE))
        .title(Span::styled("─ Help ─", Style::default().fg(cat::BLUE)))
        .style(Style::default().bg(cat::BASE));

    f.render_widget(block.clone(), modal_area);
    let inner = block.inner(modal_area);

    let sections = vec![
        ("Navigation", vec![
            ("↑↓", "move"),
            ("enter", "select"),
            ("esc", "quit"),
        ]),
        ("Actions", vec![
            ("F1", "help"),
            ("F2", "add"),
            ("F3", "edit"),
            ("F4", "delete"),
        ]),
        ("Form", vec![
            ("tab", "next field"),
            ("enter", "save"),
            ("esc", "cancel"),
        ]),
    ];

    let mut lines: Vec<Line> = Vec::new();

    for (section, shortcuts) in sections {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            section,
            Style::default().fg(cat::LAVENDER).add_modifier(Modifier::BOLD),
        )));
        for (key, desc) in shortcuts {
            lines.push(Line::from(vec![
                Span::styled(format!("  {:10}", key), Style::default().fg(cat::PEACH)),
                Span::styled(desc, Style::default().fg(cat::SUBTEXT1)),
            ]));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "esc to close",
        Style::default().fg(cat::OVERLAY0),
    )));

    f.render_widget(Paragraph::new(lines), inner);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_width = r.width * percent_x / 100;
    let popup_height = r.height * percent_y / 100;

    Rect {
        x: r.x + (r.width.saturating_sub(popup_width)) / 2,
        y: r.y + (r.height.saturating_sub(popup_height)) / 2,
        width: popup_width,
        height: popup_height,
    }
}

fn truncate_str(s: &str, max_len: usize) -> String {
    if s.chars().count() <= max_len {
        s.to_string()
    } else if max_len > 2 {
        format!("{}..", s.chars().take(max_len - 2).collect::<String>())
    } else {
        s.chars().take(max_len).collect()
    }
}

fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    if max_width == 0 {
        return vec![text.to_string()];
    }

    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        if current_line.is_empty() {
            if word.len() > max_width {
                let mut remaining = word;
                while remaining.len() > max_width {
                    lines.push(remaining[..max_width].to_string());
                    remaining = &remaining[max_width..];
                }
                current_line = remaining.to_string();
            } else {
                current_line = word.to_string();
            }
        } else if current_line.len() + 1 + word.len() <= max_width {
            current_line.push(' ');
            current_line.push_str(word);
        } else {
            lines.push(current_line);
            current_line = word.to_string();
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    if lines.is_empty() {
        lines.push(String::new());
    }

    lines
}
