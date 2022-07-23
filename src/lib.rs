mod handler;
pub mod prefabs;

use axum::body::Body;
pub use handler::PreHandler;
pub use handler::PreHandling;
use prefabs::map_error::MapError;
use prefabs::resp_result::ToRespResult;

pub type BodyPreHandling<H> = PreHandling<Body, H>;
pub type PreRespMapErrorHandling<H, E> = BodyPreHandling<ToRespResult<Body, MapError<Body, H, E>>>;
pub type PreRespHandling<H> = BodyPreHandling<ToRespResult<Body, H>>;
