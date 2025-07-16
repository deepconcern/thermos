use crate::{Request, Response, context::Context};

pub type Handler = fn(Request, Context) -> Response;
