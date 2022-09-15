use std::collections::HashMap;

use async_trait::async_trait;
use futures::Future;
use hyper::{Method, Request, Response, Body};

use crate::GenericError;

type InternalRotuer = route_recognizer::Router<Box<dyn Handler>>;

#[async_trait]
pub trait Handler: Send + Sync + 'static {
    async fn handle(&self, req: Request<Body>) -> Result<Response<Body>, GenericError>;
}

#[async_trait]
impl<F: Send + Sync + 'static, Fut> Handler for F
where 
    F: Fn(Request<Body>) -> Fut,
    Fut: Future<Output = Result<Response<Body>, GenericError>> + Send + Sync + 'static,
{
    async fn handle(&self, req: Request<Body>) -> Result<Response<Body>, GenericError> {
        self(req).await
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