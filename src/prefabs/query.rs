use std::marker::PhantomData;

use async_trait::async_trait;
use axum::extract::{rejection::QueryRejection, FromRequest, Query, RequestParts};
use serde::Deserialize;

use crate::PreHandler;

pub struct QueryParams<T>(PhantomData<T>)
where
    T: for<'de> Deserialize<'de>;

#[async_trait]
impl<B, T> PreHandler<B> for QueryParams<T>
where
    T: for<'de> Deserialize<'de>,
    B: Send,
{
    type Output = T;

    type Rejection = QueryRejection;

    async fn handling(request: &mut RequestParts<B>) -> Result<Self::Output, Self::Rejection> {
        let Query(data) = Query::from_request(request).await?;
        Ok(data)
    }
}
