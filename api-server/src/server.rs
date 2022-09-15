use std::sync::Arc;

use hyper::{service::{make_service_fn, service_fn}, Server, Response, Body, Request};

use crate::{GenericError, router::Router};

pub struct ApiServer {
    
}

async fn route(req: Request<Body>, router: Arc<Router>) -> Result<Response<Body>, GenericError> {
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    let router = router.router_map.get(&method).unwrap();
    let matched = router.recognize(&path).unwrap();
    let handler = matched.handler();

    handler.handle(req).await
}

impl ApiServer {
    pub fn new() -> Self {
        Self {
        
        }
    }

    pub async fn run(&mut self) -> Result<(), GenericError> {
        // add routing
        let mut router = Router::new();
        router.get("/hello_world", |req| async move {
            Ok(Response::new(Body::from("Hello World")))
        });

        router.get("/index", |req| async move {
            Ok(Response::new(Body::from("index")))
        });

        let shared_router = Arc::new(router);

        let addr = "127.0.0.1:1337".parse().unwrap();
        let new_service = make_service_fn(move |_| {
            let router_capture = shared_router.clone();
            async {
                Ok::<_, GenericError>(service_fn(move |req| {
                    let router = router_capture.clone();
                    async move {
                        route(req, router.clone()).await
                    }
                }))
            }
        });

        let server = Server::bind(&addr).serve(new_service);
        server.await?;

        Ok(())
    }
}