// Copyright 2024 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use std::{borrow::Cow, fmt};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    Conversion,
    Io,
    Http { code: u16, message: Option<String> },
    Network,
    Other,
}

impl ErrorKind {
    pub fn http_response(code: u16, message: Option<String>) -> Self {
        Self::Http { code, message }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Conversion => f.write_str("Conversion"),
            Self::Io => f.write_str("Io"),
            Self::Http {
                code,
                message: None,
            } => write!(f, "HTTP {code}"),
            Self::Http {
                code,
                message: Some(message),
            } => write!(f, "HTTP {code}: {message}"),
            Self::Network => f.write_str("Network"),
            Self::Other => f.write_str("Other"),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    context: ErrorContext,
}

impl Error {
    pub fn new<E>(kind: ErrorKind, error: E) -> Self
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        Self {
            context: ErrorContext::Custom {
                kind,
                error: error.into(),
            },
        }
    }

    pub fn with_message<C>(kind: ErrorKind, message: C) -> Self
    where
        C: Into<Cow<'static, str>>,
    {
        Self {
            context: ErrorContext::Message {
                kind,
                message: message.into(),
            },
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        match &self.context {
            ErrorContext::Simple(kind)
            | ErrorContext::Message { kind, .. }
            | ErrorContext::Custom { kind, .. } => kind,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.context {
            ErrorContext::Simple(kind) => write!(f, "{kind}"),
            ErrorContext::Message { message, .. } => write!(f, "{message}"),
            ErrorContext::Custom { error, .. } => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.context {
            ErrorContext::Custom { error, .. } => Some(&**error),
            _ => None,
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self {
            context: ErrorContext::Simple(kind),
        }
    }
}

impl From<url::ParseError> for Error {
    fn from(error: url::ParseError) -> Self {
        Self::new(ErrorKind::Conversion, error)
    }
}

#[derive(Debug)]
enum ErrorContext {
    Simple(ErrorKind),
    Message {
        kind: ErrorKind,
        message: Cow<'static, str>,
    },
    Custom {
        kind: ErrorKind,
        error: Box<dyn std::error::Error>,
    },
}

unsafe impl Send for ErrorContext {}
