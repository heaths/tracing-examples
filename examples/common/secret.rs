// Copyright 2024 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use clap::builder::{NonEmptyStringValueParser, TypedValueParser, ValueParserFactory};

#[derive(Clone, Debug)]
pub struct Secret(String);

impl ValueParserFactory for Secret {
    type Parser = SecretValueParser;

    fn value_parser() -> Self::Parser {
        SecretValueParser
    }
}

#[derive(Clone, Debug)]
pub struct SecretValueParser;

impl TypedValueParser for SecretValueParser {
    type Value = Secret;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let parser = NonEmptyStringValueParser::new();
        let value = parser.parse_ref(cmd, arg, value)?;

        // TODO
        Ok(Secret(value))
    }
}

impl From<Secret> for tracing_examples::models::Secret {
    fn from(secret: Secret) -> Self {
        secret.0.into()
    }
}
