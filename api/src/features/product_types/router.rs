use axum::routing;

use crate::features::product_types::handlers;
use crate::state;

pub fn router() -> axum::Router<state::AppState> {
    axum::Router::new()
        .route(
            "/types",
            routing::MethodRouter::new()
                .get(handlers::get_types)
                .post(handlers::create_type),
        )
        .route(
            "/types/{id}",
            routing::MethodRouter::new()
                .get(handlers::get_type_by_id)
                .put(handlers::update_type)
                .delete(handlers::delete_type),
        )
}
