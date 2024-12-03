use crate::impl_empty_rule;
use crate::rules::Level;
use crate::{make_rule, message::Message, result::Violation, rules::Rule};

make_rule! {
    BodyEmpty,
}
impl_empty_rule! {BodyEmpty, Error, "body", body}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_empty_body() {
        let rule = BodyEmpty::default();
        let message = Message {
            body: Some("Hello world".to_string()),
            description: Some("broadcast $destroy event on scope destruction".to_string()),
            footers: None,
            r#type: Some("feat".to_string()),
            raw: "feat(scope): broadcast $destroy event on scope destruction

Hello world"
                .to_string(),
            scope: Some("scope".to_string()),
            subject: Some("feat(scope): broadcast $destroy event on scope destruction".to_string()),
        };

        assert!(rule.validate(&message).is_none());
    }

    #[test]
    fn test_empty_body() {
        let rule = BodyEmpty::default();
        let message = Message {
            body: None,
            description: None,
            footers: None,
            r#type: Some("feat".to_string()),
            raw: "feat(scope): broadcast $destroy event on scope destruction".to_string(),
            scope: Some("scope".to_string()),
            subject: None,
        };

        let violation = rule.validate(&message);
        assert!(violation.is_some());
        assert_eq!(violation.clone().unwrap().level, Level::Error);
        assert_eq!(violation.unwrap().message, "body is empty".to_string());
    }
}
