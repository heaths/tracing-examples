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

pub struct ExampleClient {
    endpoint: Url,
    models: Arc<Mutex<HashMap<String, Model>>>,
}

impl ExampleClient {
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
            .field("client")
            .is_none_or(|name| name.name() == "ExampleClient")
        {
            span = info_span!(target: "ExampleClient::get_model", "get_model", client = "ExampleClient");
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
            .field("client")
            .is_none_or(|name| name.name() == "ExampleClient")
        {
            span = info_span!(target: "ExampleClient::create_or_update_model", "create_or_update_model", client = "ExampleClient");
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
}

impl fmt::Debug for ExampleClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExampleClient")
            .field("endpoint", &self.endpoint)
            .finish()
    }
}
