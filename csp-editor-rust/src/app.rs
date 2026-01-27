use crate::csp::{is_valid_value, CspPolicy, DIRECTIVES};

/// Different input modes for the application
#[derive(Clone, Copy, PartialEq)]
pub enum InputMode {
    Normal,
    Import,
    AddValue,
    SelectDirective,
}

/// Different focus areas in the UI
#[derive(Clone, Copy, PartialEq)]
pub enum Focus {
    DirectiveList,
    ValueList,
    Import,
    DirectiveSelect,
    ValueInput,
}

/// Application state
pub struct App {
    pub policy: CspPolicy,
    pub input_mode: InputMode,
    pub focus: Focus,
    pub import_input: String,
    pub value_input: String,
    pub selected_directive_index: usize,
    pub selected_policy_directive: usize,
    pub selected_value_index: usize,
    pub message: Option<(String, bool)>, // (message, is_success)
    pub should_quit: bool,
    pub cursor_position: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            policy: CspPolicy::new(),
            input_mode: InputMode::Normal,
            focus: Focus::DirectiveSelect,
            import_input: String::new(),
            value_input: String::new(),
            selected_directive_index: 0,
            selected_policy_directive: 0,
            selected_value_index: 0,
            message: None,
            should_quit: false,
            cursor_position: 0,
        }
    }

    pub fn get_selected_directive(&self) -> &str {
        DIRECTIVES[self.selected_directive_index]
    }

    pub fn next_directive(&mut self) {
        self.selected_directive_index = (self.selected_directive_index + 1) % DIRECTIVES.len();
    }

    pub fn previous_directive(&mut self) {
        if self.selected_directive_index == 0 {
            self.selected_directive_index = DIRECTIVES.len() - 1;
        } else {
            self.selected_directive_index -= 1;
        }
    }

    pub fn next_policy_directive(&mut self) {
        let count = self.policy.get_directives().len();
        if count > 0 {
            self.selected_policy_directive = (self.selected_policy_directive + 1) % count;
            self.selected_value_index = 0;
        }
    }

    pub fn previous_policy_directive(&mut self) {
        let count = self.policy.get_directives().len();
        if count > 0 {
            if self.selected_policy_directive == 0 {
                self.selected_policy_directive = count - 1;
            } else {
                self.selected_policy_directive -= 1;
            }
            self.selected_value_index = 0;
        }
    }

    pub fn next_value(&mut self) {
        let directives = self.policy.get_directives();
        if let Some((_, values)) = directives.get(self.selected_policy_directive) {
            let count = values.len();
            if count > 0 {
                self.selected_value_index = (self.selected_value_index + 1) % count;
            }
        }
    }

    pub fn previous_value(&mut self) {
        let directives = self.policy.get_directives();
        if let Some((_, values)) = directives.get(self.selected_policy_directive) {
            let count = values.len();
            if count > 0 {
                if self.selected_value_index == 0 {
                    self.selected_value_index = count - 1;
                } else {
                    self.selected_value_index -= 1;
                }
            }
        }
    }

    pub fn add_value(&mut self) {
        let value = self.value_input.trim().to_string();
        if value.is_empty() {
            self.set_error("Please enter a value");
            return;
        }

        if !is_valid_value(&value) {
            self.set_error("Invalid value format. Use valid domains, keywords like 'self', or wildcards");
            return;
        }

        let directive = self.get_selected_directive().to_string();
        self.policy.add_value(&directive, &value);
        self.value_input.clear();
        self.cursor_position = 0;
        self.set_success(&format!("Added '{}' to {}", value, directive));
    }

    pub fn delete_selected_value(&mut self) {
        let directives = self.policy.get_directives();
        if let Some((directive, values)) = directives.get(self.selected_policy_directive) {
            let values_vec: Vec<&String> = values.iter().collect();
            if let Some(value) = values_vec.get(self.selected_value_index) {
                let directive = directive.clone();
                let value = (*value).clone();
                self.policy.remove_value(&directive, &value);
                self.set_success(&format!("Removed '{}' from {}", value, directive));

                // Adjust selection if needed
                let new_count = self.policy.get_directives().len();
                if self.selected_policy_directive >= new_count && new_count > 0 {
                    self.selected_policy_directive = new_count - 1;
                }
                self.selected_value_index = 0;
            }
        }
    }

    pub fn import_csp(&mut self) {
        let input = self.import_input.trim().to_string();
        if input.is_empty() {
            self.set_error("Please enter a CSP string to import");
            return;
        }

        match CspPolicy::parse(&input) {
            Ok(policy) => {
                self.policy = policy;
                self.import_input.clear();
                self.cursor_position = 0;
                self.selected_policy_directive = 0;
                self.selected_value_index = 0;
                self.set_success("CSP successfully imported!");
                self.input_mode = InputMode::Normal;
                self.focus = Focus::DirectiveSelect;
            }
            Err(e) => {
                self.set_error(&format!("Error parsing CSP: {}", e));
            }
        }
    }

    pub fn get_csp_string(&self) -> String {
        self.policy.to_string()
    }

    pub fn copy_to_clipboard(&mut self) {
        let csp = self.get_csp_string();
        if csp.is_empty() {
            self.set_error("Nothing to copy - CSP is empty");
            return;
        }

        #[cfg(feature = "clipboard")]
        {
            use copypasta::{ClipboardContext, ClipboardProvider};
            match ClipboardContext::new() {
                Ok(mut ctx) => match ctx.set_contents(csp) {
                    Ok(_) => self.set_success("Copied to clipboard!"),
                    Err(e) => self.set_error(&format!("Failed to copy: {}", e)),
                },
                Err(e) => self.set_error(&format!("Clipboard unavailable: {}", e)),
            }
        }

        #[cfg(not(feature = "clipboard"))]
        {
            self.set_success("CSP string displayed above (clipboard not available in this build)");
        }
    }

    pub fn clear_policy(&mut self) {
        self.policy.clear();
        self.selected_policy_directive = 0;
        self.selected_value_index = 0;
        self.set_success("Policy cleared");
    }

    fn set_error(&mut self, msg: &str) {
        self.message = Some((msg.to_string(), false));
    }

    fn set_success(&mut self, msg: &str) {
        self.message = Some((msg.to_string(), true));
    }

    pub fn clear_message(&mut self) {
        self.message = None;
    }

    pub fn enter_char(&mut self, c: char) {
        match self.focus {
            Focus::Import => {
                self.import_input.insert(self.cursor_position, c);
                self.cursor_position += 1;
            }
            Focus::ValueInput => {
                self.value_input.insert(self.cursor_position, c);
                self.cursor_position += 1;
            }
            _ => {}
        }
    }

    pub fn delete_char(&mut self) {
        match self.focus {
            Focus::Import => {
                if self.cursor_position > 0 {
                    self.cursor_position -= 1;
                    self.import_input.remove(self.cursor_position);
                }
            }
            Focus::ValueInput => {
                if self.cursor_position > 0 {
                    self.cursor_position -= 1;
                    self.value_input.remove(self.cursor_position);
                }
            }
            _ => {}
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        let len = match self.focus {
            Focus::Import => self.import_input.len(),
            Focus::ValueInput => self.value_input.len(),
            _ => 0,
        };
        if self.cursor_position < len {
            self.cursor_position += 1;
        }
    }

    pub fn move_cursor_to_start(&mut self) {
        self.cursor_position = 0;
    }

    pub fn move_cursor_to_end(&mut self) {
        self.cursor_position = match self.focus {
            Focus::Import => self.import_input.len(),
            Focus::ValueInput => self.value_input.len(),
            _ => 0,
        };
    }
}
