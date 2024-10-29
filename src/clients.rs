// Copyright 2024 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use crate::{
    error::{ErrorKind, Result},
    models::Model,
};
use std::{collections::HashMap, fmt, sync::Arc, time::Duration};
use tokio::{sync::Mutex, time::sleep};
use tracing::{info_span, Instrument};
use url::Url;
use valuable::{Fields, NamedField, NamedValues, Structable, Valuable};

pub struct ExampleClient {
    endpoint: Url,
    models: Arc<Mutex<HashMap<String, Model>>>,
}

impl ExampleClient {
    const NAMESPACE: &str = "Microsoft.Example";

    pub fn new(endpoint: impl AsRef<str>) -> Result<Self> {
        Ok(Self {
            endpoint: Url::parse(endpoint.as_ref())?,
            models: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    pub async fn get_model(&self, name: &str) -> Result<Model> {
        let mut span = tracing::Span::current();
        if span
            .field(AZ_CLIENT_FIELD.name())
            .is_none_or(|name| name.name() == stringify!(ExampleClient))
        {
            span =
                info_span!(target: "ExampleClient::get_model", "get_model", self = self.as_value());
        }
        async move {
            sleep(Duration::from_millis(100)).await;
            let models = self.models.lock().await;
            let Some(model) = models.get(name) else {
                return Err(ErrorKind::http_response(404, None).into());
            };

            Ok(model.clone())
        }
        .instrument(span)
        .await
        .inspect_err(|err|
            tracing::error!(name: "get_model", target: "ExampleClient::get_model", error = %err)
        )
    }

    pub async fn create_or_update_model(&self, model: Model) -> Result<Model> {
        let mut span = tracing::Span::current();
        if span
            .field(AZ_CLIENT_FIELD.name())
            .is_none_or(|name| name.name() == stringify!(ExampleClient))
        {
            span = info_span!(target: "ExampleClient::create_or_update_model", "create_or_update_model", self = self.as_value());
        }
        async move {
            sleep(Duration::from_millis(300)).await;
            let Some(name) = model.name.as_ref() else {
                return Err(ErrorKind::http_response(400, None).into());
            };

            let mut models = self.models.lock().await;
            models.insert(name.clone(), model.clone());

            Ok(model)
        }.instrument(span)
        .await
        .inspect_err(|err|
            tracing::error!(name: "create_or_update_model", target: "ExampleClient::create_or_update_model", error = %err)
        )
    }

    pub(crate) fn as_value(&self) -> valuable::Value<'_> {
        tracing::field::valuable(self)
    }
}

impl fmt::Debug for ExampleClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExampleClient")
            .field("endpoint", &self.endpoint)
            .finish()
    }
}

impl Valuable for ExampleClient {
    fn as_value(&self) -> valuable::Value<'_> {
        valuable::Value::Structable(self)
    }

    fn visit(&self, visit: &mut dyn valuable::Visit) {
        visit.visit_named_fields(&NamedValues::new(
            FIELDS,
            &[
                stringify!(ExampleClient).as_value(),
                Self::NAMESPACE.as_value(),
            ],
        ));
    }
}

pub(crate) const AZ_CLIENT_FIELD: NamedField = NamedField::new("az.client");
pub(crate) const AZ_NAMESPACE_FIELD: NamedField = NamedField::new("az.namespace");
static FIELDS: &[NamedField<'static>] = &[AZ_CLIENT_FIELD, AZ_NAMESPACE_FIELD];

impl Structable for ExampleClient {
    fn definition(&self) -> valuable::StructDef<'_> {
        valuable::StructDef::new_static(stringify!(ExampleClient), Fields::Named(FIELDS))
    }
}
