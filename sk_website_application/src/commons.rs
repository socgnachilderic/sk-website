use std::fmt::Debug;

use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;

#[async_trait]
pub trait BaseHandler {
    type Action: Serialize + DeserializeOwned + Debug;
    type Response: Serialize + DeserializeOwned + Debug;

    async fn execute(&self, action: Self::Action) -> Self::Response;
}
