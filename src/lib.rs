// Copyright 2024 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

pub mod clients;
pub mod error;
pub mod models;

pub use clients::ExampleClient;
pub use error::Result;
use models::{Model, Secret};
use std::future::Future;
use tracing::Level;

pub trait ExampleClientExt: private::Sealed {
    fn rotate<S: Into<Secret>>(&self, name: &str, secret: S)
        -> impl Future<Output = Result<Model>>;
}

impl ExampleClientExt for ExampleClient {
    #[tracing::instrument(target = "ExampleClient::rotate", skip_all, fields(name), err)]
    async fn rotate<S: Into<Secret>>(&self, name: &str, secret: S) -> Result<Model> {
        let mut m = self.get_model(name).await?;

        tracing::event!(Level::DEBUG, "rotate secret");
        m.rotate(secret);

        self.create_or_update_model(m).await
    }
}

mod private {
    use crate::ExampleClient;

    pub trait Sealed {}
    impl Sealed for ExampleClient {}
}
