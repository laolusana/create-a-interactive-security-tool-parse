use std::collections::HashMap;

mod parser {
    use enum_dispatch::enum_dispatch;

    #[enum_dispatch]
    pub trait SecurityRule {
        fn parse(&self, input: &str) -> bool;
    }

    #[derive(Debug)]
    pub enum RuleType {
        Whitelist,
        Blacklist,
        Regex,
    }

    #[derive(Debug)]
    pub struct WhitelistRule {
        pattern: String,
    }

    impl SecurityRule for WhitelistRule {
        fn parse(&self, input: &str) -> bool {
            self.pattern.contains(input)
        }
    }

    #[derive(Debug)]
    pub struct BlacklistRule {
        pattern: String,
    }

    impl SecurityRule for BlacklistRule {
        fn parse(&self, input: &str) -> bool {
            !self.pattern.contains(input)
        }
    }

    #[derive(Debug)]
    pub struct RegexRule {
        pattern: regex::Regex,
    }

    impl SecurityRule for RegexRule {
        fn parse(&self, input: &str) -> bool {
            self.pattern.is_match(input)
        }
    }
}

pub struct SecurityTool {
    rules: Vec<Box<dyn parser::SecurityRule>>,
}

impl SecurityTool {
    pub fn new() -> Self {
        Self { rules: vec![] }
    }

    pub fn add_rule(&mut self, rule: Box<dyn parser::SecurityRule>) {
        self.rules.push(rule);
    }

    pub fn parse_input(&self, input: &str) -> bool {
        for rule in &self.rules {
            if !rule.parse(input) {
                return false;
            }
        }
        true
    }
}

pub fn main() {
    let mut tool = SecurityTool::new();

    let whitelist_rule = parser::WhitelistRule {
        pattern: "example.com".to_string(),
    };

    let blacklist_rule = parser::BlacklistRule {
        pattern: "bad.example.com".to_string(),
    };

    let regex_rule = parser::RegexRule {
        pattern: regex::Regex::new(r"^[a-zA-Z0-9]+$").unwrap(),
    };

    tool.add_rule(Box::new(whitelist_rule));
    tool.add_rule(Box::new(blacklist_rule));
    tool.add_rule(Box::new(regex_rule));

    let input = "example.com";
    println!("Input '{}' is valid: {}", input, tool.parse_input(input));
}