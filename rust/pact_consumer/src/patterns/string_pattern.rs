//! Support for patterns which match only strings, not JSON.

use pact_matching::models::matchingrules::Category;
use std::borrow::Cow;

use super::Pattern;

/// A pattern which matches or generates a string.
#[derive(Debug)]
pub enum StringPattern {
    /// A literal string, which matches and generates itself.
    String(String),
    /// A nested pattern.
    Pattern(Box<Pattern<Matches = String>>),
}

impl StringPattern {
    /// Construct a string pattern from a type implementing `Pattern`.
    pub fn pattern<P>(pattern: P) -> StringPattern
    where
        P: Pattern<Matches = String> + 'static,
    {
        StringPattern::Pattern(Box::new(pattern))
    }
}

impl Pattern for StringPattern {
    type Matches = String;

    fn to_example(&self) -> Self::Matches {
        match *self {
            StringPattern::String(ref s) => s.to_owned(),
            StringPattern::Pattern(ref p) => p.to_example(),
        }
    }

    fn extract_matching_rules(&self, path: &str, rules_out: &mut Category) {
        match *self {
            StringPattern::String(_) => {},
            StringPattern::Pattern(ref p) => {
                p.extract_matching_rules(path, rules_out);
            }
        }
    }
}

#[test]
fn string_pattern_is_pattern() {
    use env_logger;
    use regex::Regex;

    use super::special_rules::Term;

    let _ = env_logger::init();

    // This is our pattern, combining both example data and matching rules.
    let pattern: StringPattern = Term::new(Regex::new("^[0-9]+$").unwrap(), "10").into();

    // Make sure we generate the right output.
    assert_eq!(pattern.to_example(), "10");

    // Here are our matching rules, for passing to the low-level match engine.
    let expected_rules = hashmap!(
        s!("$.query.val") => json!({ "match": "regex", "regex": "^[0-9]+$" })
    );
    let mut rules = Category::default("query");
    pattern.extract_matching_rules("val", &mut rules);
    assert_eq!(rules.to_v2_json(), expected_rules);
}

impl<'a> From<String> for StringPattern {
    fn from(s: String) -> Self {
        StringPattern::String(s)
    }
}

impl<'a> From<&'a str> for StringPattern {
    fn from(s: &'a str) -> Self {
        StringPattern::String(s.into())
    }
}

impl<'a> From<Cow<'a, str>> for StringPattern {
    fn from(s: Cow<'a, str>) -> Self {
        StringPattern::String(s.into())
    }
}

