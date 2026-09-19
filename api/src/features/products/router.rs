use axum::routing;

use crate::features::products::handlers;
use crate::state;

pub fn router() -> axum::Router<state::AppState> {
    axum::Router::new()
        .route(
            "/products",
            routing::MethodRouter::new()
                .get(handlers::get_products)
                .post(|| async move { "Create Product" }),
        )
        .route(
            "/products/{id}",
            routing::MethodRouter::new()
                .get(handlers::get_product_by_id)
                .put(|| async move { "Update one product" })
                .delete(|| async move { "Delete one product" }),
        )
}
