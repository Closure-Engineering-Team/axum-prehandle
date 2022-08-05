use std::marker::PhantomData;

use async_trait::async_trait;
use resp_result::{Nil, RespError, RespResult};

use crate::PreHandler;

pub struct ToRespResult<B, T>(PhantomData<(B, T)>)
where
    B: Send,
    T: PreHandler<B>,
    T::Rejection: RespError;

#[async_trait]
impl<B, T> PreHandler<B> for ToRespResult<B, T>
where
    B: Send,
    T: PreHandler<B>,
    T::Rejection: RespError,
{
    type Output = T::Output;

    type Rejection = RespResult<Nil, T::Rejection>;

    async fn handling(
        request: &mut axum::extract::RequestParts<B>,
    ) -> Result<Self::Output, Self::Rejection> {
        T::handling(request).await.map_err(RespResult::err)
    }
}
