use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use cosmscan_models::{db::BackendDB, storage::PersistenceStorage};
use futures::Future;
use hyper::{Body, Method, Request, Response};

use crate::{
    errors::Error,
    resputil::{self, ResponseBuilder},
    AppState, GenericError,
};

type InternalRotuer = route_recognizer::Router<Box<dyn Handler>>;

#[async_trait]
pub trait Handler: Send + Sync + 'static {
    async fn handle(
        &self,
        req: Request<Body>,
        state: AppState,
    ) -> Result<Response<Body>, GenericError>;
}

#[async_trait]
impl<F: Send + Sync + 'static, Fut> Handler for F
where
    F: Fn(Request<Body>, AppState) -> Fut,
    Fut: Future<Output = Result<Response<Body>, GenericError>> + Send + Sync + 'static,
{
    async fn handle(
        &self,
        req: Request<Body>,
        state: AppState,
    ) -> Result<Response<Body>, GenericError> {
        self(req, state).await
    }
}

pub struct Router {
    pub router_map: HashMap<Method, InternalRotuer>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            router_map: HashMap::new(),
        }
    }

    pub fn get(&mut self, path: &str, handler: impl Handler) {
        self.router_map
            .entry(Method::GET)
            .or_insert_with(InternalRotuer::new)
            .add(path, Box::new(handler));
    }

    pub fn post(&mut self, path: &str, handler: impl Handler) {
        self.router_map
            .entry(Method::POST)
            .or_insert_with(InternalRotuer::new)
            .add(path, Box::new(handler));
    }

    pub fn put(&mut self, path: &str, handler: impl Handler) {
        self.router_map
            .entry(Method::PUT)
            .or_insert_with(InternalRotuer::new)
            .add(path, Box::new(handler));
    }

    pub fn delete(&mut self, path: &str, handler: impl Handler) {
        self.router_map
            .entry(Method::DELETE)
            .or_insert_with(InternalRotuer::new)
            .add(path, Box::new(handler));
    }
}

/// route the request to the correct handler.
pub async fn route(
    req: Request<Body>,
    router: Arc<Router>,
    storage: Arc<PersistenceStorage<BackendDB>>,
    resp_builder: Arc<ResponseBuilder>,
) -> Result<Response<Body>, GenericError> {
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    let router = router
        .router_map
        .get(&method)
        .ok_or(Error::MethodNotAllowed(method.to_string()))?;

    // if router find the right handler, then call it
    // if not, return 404
    match router.recognize(&path) {
        Ok(match_info) => {
            let handler = match_info.handler();
            let params = match_info.params().to_owned();
            handler
                .handle(req, AppState::new(storage, params, resp_builder))
                .await
        }
        Err(_) => resp_builder.not_found(),
    }
}
