// Copyright 2024 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Deserialize, Serialize)]
pub struct Model {
    pub name: Option<String>,
    pub secret: Option<Secret>,
    pub version: Option<u32>,
}

impl Model {
    pub fn new(name: impl Into<String>, secret: impl Into<Secret>) -> Self {
        Self {
            name: Some(name.into()),
            secret: Some(secret.into()),
            version: Some(1),
        }
    }

    pub fn rotate(&mut self, secret: impl Into<Secret>) {
        self.secret = Some(secret.into());
        self.version = Some(self.version.unwrap_or_default() + 1);
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Secret(String);

impl Secret {
    pub fn value(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Secret")
    }
}

impl fmt::Debug for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}

impl From<&str> for Secret {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for Secret {
    fn from(value: String) -> Self {
        Self(value)
    }
}
