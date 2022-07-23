use super::StdError;
use async_trait::async_trait;
use axum::{
    body::HttpBody,
    extract::{rejection::JsonRejection, FromRequest, RequestParts},
    Json,
};
use serde::Deserialize;
use std::marker::PhantomData;

use crate::PreHandler;

pub struct JsonPayload<T>(PhantomData<T>)
where
    T: for<'de> Deserialize<'de>;

#[async_trait]
impl<T, B> PreHandler<B> for JsonPayload<T>
where
    T: for<'de> Deserialize<'de>,
    B: Send + HttpBody,
    B::Data: Send,
    B::Error: Send + StdError + Sync + 'static,
{
    type Output = T;

    type Rejection = JsonRejection;

    async fn handling(request: &mut RequestParts<B>) -> Result<Self::Output, Self::Rejection> {
        let Json(data) = Json::from_request(request).await?;
        Ok(data)
    }
}

