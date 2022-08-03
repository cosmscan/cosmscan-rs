use diesel::{PgConnection, RunQueryDsl};

use crate::schema::blocks::dsl::blocks as all_blocks;
use crate::schema::chains::dsl::chains as all_chains;
use crate::schema::events::dsl::events as all_events;
use crate::schema::transactions::dsl::transactions as all_transactions;

#[allow(dead_code)]
pub(crate) fn cleanup_db(conn: &PgConnection) {
    diesel::delete(all_chains)
        .execute(conn)
        .expect("failed to cleanup database");

    diesel::delete(all_transactions)
        .execute(conn)
        .expect("failed to cleanup database");

    diesel::delete(all_blocks)
        .execute(conn)
        .expect("failed to cleanup database");

    diesel::delete(all_events)
        .execute(conn)
        .expect("failed to cleanup database");
}
