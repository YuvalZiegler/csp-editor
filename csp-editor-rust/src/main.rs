mod app;
mod csp;
mod ui;

use std::io;
use std::time::Duration;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use app::{App, Focus};

fn main() -> Result<(), io::Error> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        // Poll for events with timeout to allow message clearing
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                // Clear message on any key press
                app.clear_message();

                match app.focus {
                    Focus::Import => handle_import_input(app, key.code, key.modifiers),
                    Focus::DirectiveSelect => handle_directive_select(app, key.code),
                    Focus::ValueInput => handle_value_input(app, key.code, key.modifiers),
                    Focus::DirectiveList | Focus::ValueList => {
                        handle_directive_list(app, key.code)
                    }
                }

                if app.should_quit {
                    return Ok(());
                }
            }
        }
    }
}

fn handle_import_input(app: &mut App, key: KeyCode, modifiers: KeyModifiers) {
    match key {
        KeyCode::Enter => {
            app.import_csp();
        }
        KeyCode::Tab => {
            app.focus = Focus::DirectiveSelect;
            app.cursor_position = 0;
        }
        KeyCode::Esc => {
            app.import_input.clear();
            app.focus = Focus::DirectiveSelect;
            app.cursor_position = 0;
        }
        KeyCode::Char('q') if modifiers.contains(KeyModifiers::CONTROL) => {
            app.should_quit = true;
        }
        KeyCode::Char(c) => {
            app.enter_char(c);
        }
        KeyCode::Backspace => {
            app.delete_char();
        }
        KeyCode::Left => {
            app.move_cursor_left();
        }
        KeyCode::Right => {
            app.move_cursor_right();
        }
        KeyCode::Home => {
            app.move_cursor_to_start();
        }
        KeyCode::End => {
            app.move_cursor_to_end();
        }
        _ => {}
    }
}

fn handle_directive_select(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Left => {
            app.previous_directive();
        }
        KeyCode::Right => {
            app.next_directive();
        }
        KeyCode::Tab => {
            app.focus = Focus::ValueInput;
            app.cursor_position = app.value_input.len();
        }
        KeyCode::BackTab => {
            app.focus = Focus::Import;
            app.cursor_position = app.import_input.len();
        }
        KeyCode::Char('q') => {
            app.should_quit = true;
        }
        KeyCode::Char('c') => {
            app.copy_to_clipboard();
        }
        KeyCode::Down => {
            if !app.policy.is_empty() {
                app.focus = Focus::DirectiveList;
            }
        }
        _ => {}
    }
}

fn handle_value_input(app: &mut App, key: KeyCode, modifiers: KeyModifiers) {
    match key {
        KeyCode::Enter => {
            app.add_value();
        }
        KeyCode::Tab => {
            if !app.policy.is_empty() {
                app.focus = Focus::DirectiveList;
            } else {
                app.focus = Focus::Import;
            }
            app.cursor_position = 0;
        }
        KeyCode::BackTab => {
            app.focus = Focus::DirectiveSelect;
        }
        KeyCode::Esc => {
            app.value_input.clear();
            app.cursor_position = 0;
        }
        KeyCode::Char('q') if modifiers.contains(KeyModifiers::CONTROL) => {
            app.should_quit = true;
        }
        KeyCode::Char(c) => {
            app.enter_char(c);
        }
        KeyCode::Backspace => {
            app.delete_char();
        }
        KeyCode::Left => {
            app.move_cursor_left();
        }
        KeyCode::Right => {
            app.move_cursor_right();
        }
        KeyCode::Home => {
            app.move_cursor_to_start();
        }
        KeyCode::End => {
            app.move_cursor_to_end();
        }
        _ => {}
    }
}

fn handle_directive_list(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Up => {
            app.previous_policy_directive();
        }
        KeyCode::Down => {
            app.next_policy_directive();
        }
        KeyCode::Left => {
            app.previous_value();
            if app.focus == Focus::DirectiveList {
                app.focus = Focus::ValueList;
            }
        }
        KeyCode::Right => {
            app.next_value();
            if app.focus == Focus::DirectiveList {
                app.focus = Focus::ValueList;
            }
        }
        KeyCode::Tab => {
            app.focus = Focus::Import;
            app.cursor_position = app.import_input.len();
        }
        KeyCode::BackTab => {
            app.focus = Focus::ValueInput;
            app.cursor_position = app.value_input.len();
        }
        KeyCode::Char('d') | KeyCode::Delete => {
            app.delete_selected_value();
            if app.policy.is_empty() {
                app.focus = Focus::DirectiveSelect;
            }
        }
        KeyCode::Char('q') => {
            app.should_quit = true;
        }
        KeyCode::Char('c') => {
            app.copy_to_clipboard();
        }
        KeyCode::Char('x') => {
            app.clear_policy();
            app.focus = Focus::DirectiveSelect;
        }
        KeyCode::Esc => {
            app.focus = Focus::DirectiveSelect;
        }
        _ => {}
    }
}
