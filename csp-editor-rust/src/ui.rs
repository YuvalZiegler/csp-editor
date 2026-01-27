use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, Focus, InputMode};
use crate::csp::DIRECTIVES;

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Length(3),  // Import section
            Constraint::Length(6),  // CSP Preview
            Constraint::Length(3),  // Directive selector
            Constraint::Length(3),  // Value input
            Constraint::Min(8),     // Directives list
            Constraint::Length(3),  // Message/status
            Constraint::Length(2),  // Help
        ])
        .split(frame.area());

    draw_title(frame, chunks[0]);
    draw_import_section(frame, app, chunks[1]);
    draw_csp_preview(frame, app, chunks[2]);
    draw_directive_selector(frame, app, chunks[3]);
    draw_value_input(frame, app, chunks[4]);
    draw_directives_list(frame, app, chunks[5]);
    draw_message(frame, app, chunks[6]);
    draw_help(frame, app, chunks[7]);
}

fn draw_title(frame: &mut Frame, area: Rect) {
    let title = Paragraph::new("CSP Policy Builder")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, area);
}

fn draw_import_section(frame: &mut Frame, app: &App, area: Rect) {
    let style = if app.focus == Focus::Import {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::Gray)
    };

    let input = Paragraph::new(app.import_input.as_str())
        .style(style)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Import CSP (Tab to focus, Enter to import)")
                .border_style(if app.focus == Focus::Import {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default()
                }),
        );
    frame.render_widget(input, area);

    // Show cursor when focused
    if app.focus == Focus::Import {
        frame.set_cursor_position((
            area.x + app.cursor_position as u16 + 1,
            area.y + 1,
        ));
    }
}

fn draw_csp_preview(frame: &mut Frame, app: &App, area: Rect) {
    let csp = app.get_csp_string();
    let char_count = csp.len();

    let preview = Paragraph::new(if csp.is_empty() {
        "(empty - add directives below)".to_string()
    } else {
        csp
    })
    .style(Style::default().fg(if char_count > 0 { Color::Green } else { Color::DarkGray }))
    .wrap(Wrap { trim: false })
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!("CSP Preview ({} chars) - Press 'c' to copy", char_count)),
    );
    frame.render_widget(preview, area);
}

fn draw_directive_selector(frame: &mut Frame, app: &App, area: Rect) {
    let style = if app.focus == Focus::DirectiveSelect {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::Gray)
    };

    let directive = app.get_selected_directive();
    let selector = Paragraph::new(format!("< {} >", directive))
        .style(style.add_modifier(Modifier::BOLD))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Select Directive (←/→ to change)")
                .border_style(if app.focus == Focus::DirectiveSelect {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default()
                }),
        );
    frame.render_widget(selector, area);
}

fn draw_value_input(frame: &mut Frame, app: &App, area: Rect) {
    let style = if app.focus == Focus::ValueInput {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::Gray)
    };

    let placeholder = if app.value_input.is_empty() && app.focus != Focus::ValueInput {
        "e.g., 'self', https://example.com, *.domain.com"
    } else {
        ""
    };

    let display_text = if app.value_input.is_empty() {
        placeholder
    } else {
        &app.value_input
    };

    let input = Paragraph::new(display_text)
        .style(if app.value_input.is_empty() && app.focus != Focus::ValueInput {
            Style::default().fg(Color::DarkGray)
        } else {
            style
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Value Input (Enter to add)")
                .border_style(if app.focus == Focus::ValueInput {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default()
                }),
        );
    frame.render_widget(input, area);

    // Show cursor when focused
    if app.focus == Focus::ValueInput {
        frame.set_cursor_position((
            area.x + app.cursor_position as u16 + 1,
            area.y + 1,
        ));
    }
}

fn draw_directives_list(frame: &mut Frame, app: &App, area: Rect) {
    let directives = app.policy.get_directives();

    if directives.is_empty() {
        let empty = Paragraph::new("No directives added yet. Select a directive and add values above.")
            .style(Style::default().fg(Color::DarkGray))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Policy Directives"),
            );
        frame.render_widget(empty, area);
        return;
    }

    let items: Vec<ListItem> = directives
        .iter()
        .enumerate()
        .map(|(i, (directive, values))| {
            let is_selected = app.focus == Focus::DirectiveList && i == app.selected_policy_directive;

            let values_vec: Vec<&String> = values.iter().collect();
            let values_display: Vec<Span> = values_vec
                .iter()
                .enumerate()
                .flat_map(|(j, v)| {
                    let is_value_selected = is_selected
                        && app.focus == Focus::ValueList
                        && j == app.selected_value_index;

                    let style = if is_value_selected {
                        Style::default().bg(Color::Red).fg(Color::White)
                    } else {
                        Style::default().fg(Color::Blue)
                    };

                    vec![
                        Span::styled(v.as_str(), style),
                        Span::raw(" "),
                    ]
                })
                .collect();

            let mut line_spans = vec![
                Span::styled(
                    format!("{}: ", directive),
                    Style::default()
                        .fg(if is_selected { Color::Yellow } else { Color::Cyan })
                        .add_modifier(Modifier::BOLD),
                ),
            ];
            line_spans.extend(values_display);

            ListItem::new(Line::from(line_spans)).style(
                if is_selected && app.focus == Focus::DirectiveList {
                    Style::default().bg(Color::DarkGray)
                } else {
                    Style::default()
                },
            )
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Policy Directives (↑/↓ to navigate, 'd' to delete value)")
                .border_style(if app.focus == Focus::DirectiveList || app.focus == Focus::ValueList {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default()
                }),
        );
    frame.render_widget(list, area);
}

fn draw_message(frame: &mut Frame, app: &App, area: Rect) {
    let (text, style) = match &app.message {
        Some((msg, is_success)) => {
            if *is_success {
                (msg.clone(), Style::default().fg(Color::Green))
            } else {
                (msg.clone(), Style::default().fg(Color::Red))
            }
        }
        None => (String::new(), Style::default()),
    };

    let message = Paragraph::new(text)
        .style(style)
        .block(Block::default().borders(Borders::ALL).title("Status"));
    frame.render_widget(message, area);
}

fn draw_help(frame: &mut Frame, app: &App, area: Rect) {
    let help_text = match app.focus {
        Focus::Import => "Enter: Import | Tab: Next field | Esc: Cancel | q: Quit",
        Focus::DirectiveSelect => "←/→: Change directive | Tab: Next field | q: Quit",
        Focus::ValueInput => "Enter: Add value | Tab: Next field | q: Quit",
        Focus::DirectiveList | Focus::ValueList => "↑/↓: Navigate | ←/→: Values | d: Delete | Tab: Next | c: Copy | x: Clear | q: Quit",
    };

    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(help, area);
}
