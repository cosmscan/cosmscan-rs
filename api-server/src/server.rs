use std::sync::Arc;

use cosmscan_models::{
    db::{BackendDB, Database},
    storage::PersistenceStorage,
};
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Response, Server,
};
use log::{error, info};

use crate::{
    handlers,
    resputil::ResponseBuilder,
    router::{self, Router},
    Config, GenericError,
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

        // construct response builder
        let resp_builder = ResponseBuilder::new(self.config.server.allowed_host.clone());
        let shared_resp_builder = Arc::new(resp_builder);

        // add routing
        let shared_router = Arc::new(self.router());
        let new_service = make_service_fn(move |_| {
            let router_capture = shared_router.clone();
            let storage_capture = shared_storage.clone();
            let resp_builder_capture = shared_resp_builder.clone();

            async {
                Ok::<_, GenericError>(service_fn(move |req| {
                    let router = router_capture.clone();
                    let storage = storage_capture.clone();
                    let resp_builder = resp_builder_capture.clone();

                    async move {
                        let result: Result<Response<Body>, GenericError> = match router::route(
                            req,
                            router.clone(),
                            storage.clone(),
                            resp_builder.clone(),
                        )
                        .await
                        {
                            Ok(res) => Ok(res),
                            Err(e) => {
                                error!("Internal Server Error: {}", e);
                                resp_builder.internal_error()
                            }
                        };
                        result
                    }
                }))
            }
        });

        // serve server
        let addr_string = format!("{}:{}", self.config.server.host, self.config.server.port);
        let addr = addr_string.parse().unwrap();
        let server = Server::bind(&addr).serve(new_service);
        info!("Server listening on http://{}", addr);
        server.await?;

        Ok(())
    }

    pub fn router(&self) -> Router {
        let mut router = Router::new();

        router.get("/api/chains/all", handlers::all_chains);
        router.get("/api/block/latest_block/:chain_id", handlers::latest_block);
        router.get("/api/block/list/:chain_id", handlers::block_list);
        router.get("/api/block/:chain_id/:block_height", handlers::get_block);
        router.get("/api/tx/:tx_hash", handlers::transaction_by_hash);
        router.get(
            "/api/tx/list/:chain_id/at/:block_height",
            handlers::transaction_list_in_block,
        );

        router
    }
}
