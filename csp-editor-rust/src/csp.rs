use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

/// Available CSP directives
pub const DIRECTIVES: &[&str] = &[
    "default-src",
    "script-src",
    "style-src",
    "img-src",
    "connect-src",
    "font-src",
    "frame-src",
    "media-src",
    "object-src",
    "manifest-src",
    "base-uri",
    "form-action",
    "frame-ancestors",
    "report-uri",
    "report-to",
    "worker-src",
    "child-src",
    "navigate-to",
    "prefetch-src",
];

lazy_static! {
    static ref VALID_PATTERNS: Vec<Regex> = vec![
        // CSP Keywords
        Regex::new(r"^'self'$").unwrap(),
        Regex::new(r"^'unsafe-inline'$").unwrap(),
        Regex::new(r"^'unsafe-eval'$").unwrap(),
        Regex::new(r"^'unsafe-hashes'$").unwrap(),
        Regex::new(r"^'strict-dynamic'$").unwrap(),
        Regex::new(r"^'report-sample'$").unwrap(),
        Regex::new(r"^'none'$").unwrap(),
        Regex::new(r"^'wasm-unsafe-eval'$").unwrap(),

        // URLs and Domains
        Regex::new(r"^https?://[\w\-.]+(:\d+)?(/[\w\-./]*)*$").unwrap(),
        Regex::new(r"^[\w][\w\-.]*\.[a-z]{2,}(:\d+)?(/[\w\-./]*)*$").unwrap(),

        // Wildcards
        Regex::new(r"^\*$").unwrap(),
        Regex::new(r"^\*\.[\w][\w\-.]*\.[a-z]{2,}$").unwrap(),
        Regex::new(r"^[\w][\w\-.]*\.\*$").unwrap(),

        // WebSocket URLs
        Regex::new(r"^wss?://[\w\-.]+(:\d+)?(/[\w\-./]*)*$").unwrap(),

        // Special Schemes
        Regex::new(r"^data:$").unwrap(),
        Regex::new(r"^blob:$").unwrap(),
        Regex::new(r"^filesystem:$").unwrap(),
        Regex::new(r"^mediastream:$").unwrap(),

        // Nonce and Hashes
        Regex::new(r"^'nonce-[\w+/=]+'$").unwrap(),
        Regex::new(r"^'sha256-[\w+/=]+'$").unwrap(),
        Regex::new(r"^'sha384-[\w+/=]+'$").unwrap(),
        Regex::new(r"^'sha512-[\w+/=]+'$").unwrap(),
    ];
}

/// Validates a CSP value against known patterns
pub fn is_valid_value(value: &str) -> bool {
    VALID_PATTERNS.iter().any(|pattern| pattern.is_match(value))
}

/// Represents a CSP policy with directives and their values
#[derive(Default, Clone)]
pub struct CspPolicy {
    directives: HashMap<String, HashSet<String>>,
    directive_order: Vec<String>,
}

impl CspPolicy {
    pub fn new() -> Self {
        Self {
            directives: HashMap::new(),
            directive_order: Vec::new(),
        }
    }

    /// Parse a CSP string into the policy
    pub fn parse(csp_string: &str) -> Result<Self, String> {
        let mut policy = Self::new();

        let directive_groups: Vec<&str> = csp_string
            .split(';')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        for group in directive_groups {
            let parts: Vec<&str> = group.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            let directive = parts[0].to_string();
            let values: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();

            for value in values {
                if !value.is_empty() {
                    policy.add_value(&directive, &value);
                }
            }

            // Ensure directive exists even with no values
            if !policy.directives.contains_key(&directive) {
                policy.directives.insert(directive.clone(), HashSet::new());
                policy.directive_order.push(directive);
            }
        }

        Ok(policy)
    }

    /// Add a value to a directive
    pub fn add_value(&mut self, directive: &str, value: &str) {
        if !self.directives.contains_key(directive) {
            self.directives.insert(directive.to_string(), HashSet::new());
            self.directive_order.push(directive.to_string());
        }
        self.directives
            .get_mut(directive)
            .unwrap()
            .insert(value.to_string());
    }

    /// Remove a value from a directive
    pub fn remove_value(&mut self, directive: &str, value: &str) {
        if let Some(values) = self.directives.get_mut(directive) {
            values.remove(value);
            if values.is_empty() {
                self.directives.remove(directive);
                self.directive_order.retain(|d| d != directive);
            }
        }
    }

    /// Clear all directives
    pub fn clear(&mut self) {
        self.directives.clear();
        self.directive_order.clear();
    }

    /// Get all directives in order
    pub fn get_directives(&self) -> Vec<(&String, &HashSet<String>)> {
        self.directive_order
            .iter()
            .filter_map(|d| self.directives.get(d).map(|v| (d, v)))
            .collect()
    }

    /// Get values for a specific directive
    pub fn get_values(&self, directive: &str) -> Option<&HashSet<String>> {
        self.directives.get(directive)
    }

    /// Generate the CSP string
    pub fn to_string(&self) -> String {
        self.directive_order
            .iter()
            .filter_map(|directive| {
                self.directives.get(directive).map(|values| {
                    let values_str: Vec<&str> = values.iter().map(|s| s.as_str()).collect();
                    if values_str.is_empty() {
                        directive.clone()
                    } else {
                        format!("{} {}", directive, values_str.join(" "))
                    }
                })
            })
            .collect::<Vec<String>>()
            .join("; ")
    }

    /// Get the character count of the CSP string
    pub fn char_count(&self) -> usize {
        self.to_string().len()
    }

    /// Check if policy is empty
    pub fn is_empty(&self) -> bool {
        self.directives.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_values() {
        assert!(is_valid_value("'self'"));
        assert!(is_valid_value("'unsafe-inline'"));
        assert!(is_valid_value("https://example.com"));
        assert!(is_valid_value("*.example.com"));
        assert!(is_valid_value("data:"));
        assert!(is_valid_value("'nonce-abc123='"));
        assert!(is_valid_value("'sha256-abc123='"));
    }

    #[test]
    fn test_invalid_values() {
        assert!(!is_valid_value("invalid"));
        assert!(!is_valid_value("self")); // missing quotes
        assert!(!is_valid_value("javascript:"));
    }

    #[test]
    fn test_parse_csp() {
        let csp = "default-src 'self'; script-src 'self' 'unsafe-inline'";
        let policy = CspPolicy::parse(csp).unwrap();

        assert!(policy.get_values("default-src").unwrap().contains("'self'"));
        assert!(policy.get_values("script-src").unwrap().contains("'self'"));
        assert!(policy.get_values("script-src").unwrap().contains("'unsafe-inline'"));
    }

    #[test]
    fn test_add_remove_value() {
        let mut policy = CspPolicy::new();
        policy.add_value("default-src", "'self'");
        assert!(policy.get_values("default-src").unwrap().contains("'self'"));

        policy.remove_value("default-src", "'self'");
        assert!(policy.get_values("default-src").is_none());
    }
}
