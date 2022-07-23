use std::marker::PhantomData;

use async_trait::async_trait;
use axum::extract::{rejection::PathRejection, FromRequest, Path, RequestParts};
use serde::Deserialize;

use crate::PreHandler;

pub struct PathValue<T>(PhantomData<T>)
where
    T: for<'de> Deserialize<'de> + Send;

#[async_trait]
impl<B, T> PreHandler<B> for PathValue<T>
where
    B: Send,
    T: for<'de> Deserialize<'de> + Send,
{
    type Output = T;

    type Rejection = PathRejection;

    async fn handling(request: &mut RequestParts<B>) -> Result<Self::Output, Self::Rejection> {
        let Path(data) = Path::from_request(request).await?;
        Ok(data)
    }
}
