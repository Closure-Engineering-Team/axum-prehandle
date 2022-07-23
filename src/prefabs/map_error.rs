use std::marker::PhantomData;

use async_trait::async_trait;
use axum::extract::RequestParts;

use crate::PreHandler;

pub struct MapError<B: Send, H: PreHandler<B>, R: From<H::Rejection>>(PhantomData<(B, H, R)>);

#[async_trait]
impl<B, H, R> PreHandler<B> for MapError<B, H, R>
where
    B: Send,
    H: PreHandler<B>,
    R: From<H::Rejection>,
{
    type Output = H::Output;

    type Rejection = R;

    async fn handling(request: &mut RequestParts<B>) -> Result<Self::Output, Self::Rejection> {
        H::handling(request).await.map_err(R::from)
    }
}
