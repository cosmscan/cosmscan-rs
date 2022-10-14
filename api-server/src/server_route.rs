use crate::{server::ApiServer, router::{RouterRegister, Router}, handlers};

impl RouterRegister for ApiServer {
    fn router(&self) -> Router {
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