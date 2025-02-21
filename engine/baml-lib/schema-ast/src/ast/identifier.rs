use baml_types::{BamlMediaType, TypeValue};

use super::{Span, WithName, WithSpan};
use std::fmt::Display;

/// An identifier the refers to a field or type in a different location.
#[derive(Debug, Clone, PartialEq)]
pub struct RefIdentifier {
    pub path: Vec<String>,
    /// The identifier contents.
    pub name: String,
    pub full_name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Identifier {
    /// Starts with env.*
    ENV(String, Span),
    /// The path to a Local Identifer + the local identifer. Separated by '.'
    Ref(RefIdentifier, Span),
    /// A string without spaces or '.' Always starts with a letter. May contain numbers
    Local(String, Span),
    /// A string without spaces, but contains '-'
    String(String, Span),
    /// Something that cannot be used for anything.
    Invalid(String, Span),
}

impl Identifier {
    pub fn to_string(&self) -> String {
        match self {
            Identifier::ENV(s, _) => format!("env.{}", s),
            Identifier::Ref(ref_identifier, _) => ref_identifier.full_name.clone(),
            Identifier::Local(s, _) => s.clone(),
            Identifier::String(s, _) => s.clone(),
            Identifier::Invalid(s, _) => s.clone(),
        }
    }
    pub fn is_valid_type(&self) -> bool {
        match self {
            Identifier::ENV(_, _) => false,
            Identifier::Ref(_, _) => true,
            Identifier::Local(_, _) => true,
            Identifier::String(_, _) => false,
            Identifier::Invalid(_, _) => false,
        }
    }

    pub fn is_valid_type_name(&self) -> bool {
        match self {
            Identifier::ENV(_, _) => false,
            Identifier::Ref(_, _) => true,
            Identifier::Local(_, _) => true,

            Identifier::String(_, _) => false,
            Identifier::Invalid(_, _) => false,
        }
    }

    pub fn is_valid_value(&self) -> bool {
        match self {
            Identifier::ENV(_, _) => true,
            Identifier::Local(_, _) => true,
            Identifier::String(_, _) => true,
            Identifier::Ref(_, _) => false,

            Identifier::Invalid(_, _) => false,
        }
    }
}

impl WithSpan for Identifier {
    fn span(&self) -> &Span {
        match self {
            Identifier::ENV(_, span) => span,
            Identifier::Ref(_, span) => span,
            Identifier::Local(_, span) => span,

            Identifier::String(_, span) => span,
            Identifier::Invalid(_, span) => span,
        }
    }
}

impl WithName for Identifier {
    fn name(&self) -> &str {
        match self {
            Identifier::Ref(ref_identifier, _) => &ref_identifier.full_name,
            Identifier::Local(name, _) => name,
            Identifier::String(s, _) => s,
            Identifier::ENV(name, _) => name,
            Identifier::Invalid(name, _) => name,
        }
    }
}

impl From<(&str, Span)> for Identifier {
    fn from((s, span): (&str, Span)) -> Self {
        match s {
            s if s.starts_with("env.") => Identifier::ENV(s[4..].to_string(), span),
            s if s.contains('.') => Identifier::Ref(
                RefIdentifier {
                    path: s.split('.').map(|s| s.to_string()).collect::<Vec<_>>()
                        [..s.split('.').count() - 1]
                        .to_vec(),
                    name: s.split('.').last().unwrap().to_string(),
                    full_name: s.to_string(),
                },
                span,
            ),
            "env" => Identifier::Invalid("env".into(), span),
            other if other.contains('-') => Identifier::String(other.to_string(), span),
            other => Identifier::Local(other.to_string(), span),
        }
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
