use crate::{message::Message, result::Violation};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use self::{
    body_empty::BodyEmpty, body_max_length::BodyMaxLength, description_empty::DescriptionEmpty,
    description_format::DescriptionFormat, description_max_length::DescriptionMaxLength,
    footers_empty::FootersEmpty, r#type::Type, scope::Scope, scope_empty::ScopeEmpty,
    scope_format::ScopeFormat, scope_max_length::ScopeMaxLength, subject_empty::SubjectEmpty,
    type_empty::TypeEmpty, type_format::TypeFormat, type_max_length::TypeMaxLength,
};

pub mod body_empty;
pub mod body_max_length;
pub mod description_empty;
pub mod description_format;
pub mod description_max_length;
pub mod footers_empty;
pub mod scope;
pub mod scope_empty;
pub mod scope_format;
pub mod scope_max_length;
pub mod subject_empty;
pub mod r#type;
pub mod type_empty;
pub mod type_format;
pub mod type_max_length;

/// Rules represents the rules of commitlint.
/// See: https://commitlint.js.org/reference/rules.html
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Rules {
    #[serde(rename = "body-empty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_empty: Option<BodyEmpty>,

    #[serde(rename = "body-max-length")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_max_length: Option<BodyMaxLength>,

    #[serde(rename = "description-empty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_empty: Option<DescriptionEmpty>,

    #[serde(rename = "description-format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_format: Option<DescriptionFormat>,

    #[serde(rename = "description-max-length")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_max_length: Option<DescriptionMaxLength>,

    #[serde(rename = "footers-empty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footers_empty: Option<FootersEmpty>,

    #[serde(rename = "scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Scope>,

    #[serde(rename = "scope-empty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_empty: Option<ScopeEmpty>,

    #[serde(rename = "scope-format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_format: Option<ScopeFormat>,

    #[serde(rename = "scope-max-length")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_max_length: Option<ScopeMaxLength>,

    #[serde(rename = "subject-empty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_empty: Option<SubjectEmpty>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,

    #[serde(rename = "type-empty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_empty: Option<TypeEmpty>,

    #[serde(rename = "type-format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_format: Option<TypeFormat>,

    #[serde(rename = "type-max-length")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_max_length: Option<TypeMaxLength>,
}

/// Rule is a collection of rules.
impl Rules {
    pub fn validate(&self, message: &Message) -> Vec<Violation> {
        let mut results = Vec::new();

        if let Some(rule) = &self.body_empty {
            if let Some(validation) = rule.validate(message) {
                results.push(validation);
            }
        }

        if let Some(rule) = &self.body_max_length {
            if let Some(validation) = rule.validate(message) {
                results.push(validation);
            }
        }

        if let Some(rule) = &self.description_empty {
            if let Some(validation) = rule.validate(message) {
                results.push(validation);
            }
        }

        if let Some(rule) = &self.description_format {
            if let Some(validation) = rule.validate(message) {
                results.push(validation);
            }
        }

        if let Some(rule) = &self.description_max_length {
            if let Some(validation) = rule.validate(message) {
                results.push(validation);
            }
        }

        if let Some(rule) = &self.scope {
            if let Some(validation) = rule.validate(message) {
                results.push(validation);
            }
        }

        if let Some(rule) = &self.scope_empty {
            if let Some(validation) = rule.validate(message) {
                results.push(validation);
            }
        }

        if let Some(rule) = &self.scope_format {
            if let Some(validation) = rule.validate(message) {
                results.push(validation);
            }
        }

        if let Some(rule) = &self.scope_max_length {
            if let Some(validation) = rule.validate(message) {
                results.push(validation);
            }
        }

        if let Some(rule) = &self.subject_empty {
            if let Some(validation) = rule.validate(message) {
                results.push(validation);
            }
        }

        if let Some(rule) = &self.r#type {
            if let Some(validation) = rule.validate(message) {
                results.push(validation);
            }
        }

        if let Some(rule) = &self.type_empty {
            if let Some(validation) = rule.validate(message) {
                results.push(validation);
            }
        }

        if let Some(rule) = &self.type_format {
            if let Some(validation) = rule.validate(message) {
                results.push(validation);
            }
        }

        if let Some(rule) = &self.type_max_length {
            if let Some(validation) = rule.validate(message) {
                results.push(validation);
            }
        }

        results
    }
}

/// Default implementation of Rules.
/// If no config files are specified, this will be used.
impl Default for Rules {
    fn default() -> Self {
        Self {
            body_empty: None,
            body_max_length: None,
            description_empty: DescriptionEmpty::default().into(),
            description_format: None,
            description_max_length: None,
            footers_empty: None,
            scope: None,
            scope_empty: None,
            scope_format: None,
            scope_max_length: None,
            subject_empty: SubjectEmpty::default().into(),
            r#type: None,
            type_empty: TypeEmpty::default().into(),
            type_format: None,
            type_max_length: None,
        }
    }
}

/// Rule trait represents a rule that can be applied to a text.
pub trait Rule: Default {
    /// The name of the rule.
    /// Note that it should be unique
    const NAME: &'static str;

    /// The message to display when the rule fails.
    fn message(&self, message: &Message) -> String;

    /// The level of the rule.
    const LEVEL: Level;

    /// Validate the given text.
    fn validate(&self, message: &Message) -> Option<Violation>;
}

/// Level represents the level of a rule.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum Level {
    #[serde(rename = "error")]
    Error,

    #[serde(rename = "ignore")]
    Ignore,

    #[serde(rename = "warning")]
    Warning,
}
/// Create a struct with length field that should impl [Rule]
#[macro_export]
macro_rules! make_length_rule {
    (
        $ident:ident,
        $length_of_what:literal
    ) => {
        $crate::make_rule! {
            $ident,
            #[doc = concat!("Length represents the maximum length of the ",stringify!($length_of_what),".")]
            length: usize
        }
    };
}
/// Create a struct with format field that should impl [Rule]
#[macro_export]
macro_rules! make_format_rule {
    (
        $ident:ident,
        $format_of_what:literal
    ) => {
        $crate::make_rule! {
            $ident,
            #[doc = concat!("Format represents the format of the ",stringify!($format_of_what),".")]
            format: Option<String>
        }
    };
}
/// Create a struct with options field that should impl [Rule]
#[macro_export]
macro_rules! make_options_rule {
    (
        $ident:ident,
        $options_what: literal,
        $(
            $(
                #[$field_meta:meta]
            )*
            $field_name:ident: $field_type:ty
        ),*) => {
            $crate::make_rule! {
                $ident,
                $(
                    $(
                        #[$field_meta]
                    )*
                    $field_name: $field_type,

                ),*
                #[doc = concat!("Options represents the options of the rule. If the option is empty, it means that no ",stringify!($options_what)," is allowed.")]
                options: Vec<String>
        }
        };
}

/// Create a struct that should impl [Rule]
#[macro_export]
macro_rules! make_rule {
    (
        $ident:ident,
        $(
            $(
                #[$field_meta:meta]
            )*
            $field_name:ident: $field_type:ty
        ),*) => { paste::paste! {
        #[doc = "[" $ident "] represents the [`"[<$ident:dash>]"`](https://keisukeyamashita.github.io/commitlint-rs/rules/"[<$ident:dash>]") rule."]
        #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
        #[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
        pub struct $ident {
            /// Level represents the level of the rule.
            ///
            // Note that currently the default literal is not supported.
            // See: https://github.com/serde-rs/serde/issues/368
            level: Option<Level>,
            $(
                $(
                    #[$field_meta]
                )*
                $field_name: $field_type
            ),*
        }
    }
};
}
