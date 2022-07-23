use std::{
    future::Future,
    ops::{Deref, DerefMut},
};

use async_trait::async_trait;
use axum::{
    extract::{FromRequest, RequestParts},
    response::IntoResponse,
};

/// handle data before entry request handle
pub trait PreHandler<B>
where
    B: Send,
{
    /// the handle result
    type Output;
    /// the error
    type Rejection;
    /// future type
    type Future: Future<Output = Result<Self::Output, Self::Rejection>> + Send;

    /// handler
    fn handling(request: &mut RequestParts<B>) -> Self::Future;
}

pub struct PreHandling<B, H>(pub H::Output)
where
    B: Send,
    H: PreHandler<B>,
    H::Rejection: IntoResponse;

impl<B, H> PreHandling<B, H>
where
    B: Send,
    H: PreHandler<B>,
    H::Rejection: IntoResponse,
{
    /// unwrap itself
    pub fn into_inner(self) -> H::Output {
        self.0
    }
}

impl<B, H> Deref for PreHandling<B, H>
where
    B: Send,
    H: PreHandler<B>,
    H::Rejection: IntoResponse,
{
    type Target = H::Output;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<B, H> DerefMut for PreHandling<B, H>
where
    B: Send,
    H: PreHandler<B>,
    H::Rejection: IntoResponse,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[async_trait]
impl<B, H> FromRequest<B> for PreHandling<B, H>
where
    B: Send,
    H: PreHandler<B>,
    H::Rejection: IntoResponse,
{
    type Rejection = H::Rejection;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        Ok(PreHandling(H::handling(req).await?))
    }
}
