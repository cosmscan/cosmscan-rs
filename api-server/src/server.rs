use std::sync::Arc;

use cosmscan_models::{
    db::{BackendDB, Database},
    storage::PersistenceStorage,
};
use hyper::{
    header,
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server, StatusCode,
};
use log::info;

use crate::{
    handlers,
    router::{self, Router},
    AppState, Config, GenericError,
};

pub struct ApiServer {
    pub config: Config,
}

impl ApiServer {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// run the server on the given address & port
    pub async fn run(&self) -> Result<(), GenericError> {
        // connect to the database
        let mut db = BackendDB::new(self.config.db.clone());
        db.connect();
        let storage = PersistenceStorage::new(db);
        let shared_storage = Arc::new(storage);

        // add routing
        let shared_router = Arc::new(self.router());

        let addr_string = format!("{}:{}", self.config.server.host, self.config.server.port);
        let addr = addr_string.parse().unwrap();
        let new_service = make_service_fn(move |_| {
            let router_capture = shared_router.clone();
            let storage_capture = shared_storage.clone();

            async {
                Ok::<_, GenericError>(service_fn(move |req| {
                    let router = router_capture.clone();
                    let storage = storage_capture.clone();

                    async move {
                        let result: Result<Response<Body>, GenericError> =
                            match router::route(req, router.clone(), storage.clone()).await {
                                Ok(res) => Ok(res),
                                Err(_) => {
                                    let response = Response::builder()
                                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                                        .header(header::CONTENT_TYPE, "application/json")
                                        .body(Body::from(
                                            "{ \"error\": \"Internal Server Error\" }",
                                        ))?;
                                    Ok(response)
                                }
                            };
                        result
                    }
                }))
            }
        });

        let server = Server::bind(&addr).serve(new_service);
        info!("Server listening on http://{}", addr);
        server.await?;

        Ok(())
    }

    pub fn router(&self) -> Router {
        let mut router = Router::new();

        router.get("/hello_world", handlers::handle_hello_world);
        router.get("/api/block/:block_height", handlers::get_block);

        router
    }
}
