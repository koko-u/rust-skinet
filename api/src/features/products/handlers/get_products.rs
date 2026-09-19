use axum::extract;

use crate::errors;
use crate::features::products::models;
use crate::features::products::products_repository::ProductsRepository;
use crate::features::products::responses;
use crate::shared;
use crate::state;

#[utoipa::path(
    get,
    path = "/api/products",
    description = "Get All Products",
    tag = "Products",
    responses(
        (status = 200, description = "List of Products", body = Vec<responses::Product>),
        (status = 500, description = "Internal server error", body = shared::responses::ProblemDetails),
    )
)]
pub async fn get_products(
    extract::State(state): extract::State<state::AppState>,
) -> Result<axum::Json<Vec<responses::Product>>, errors::ApiError> {
    let mut conn = state.pool.acquire().await?;
    let repo = ProductsRepository::from(conn.as_mut());
    let products = repo.select_all().await?;

    let response = products
        .into_iter()
        .map(models::Product::from)
        .map(responses::Product::from)
        .collect();

    Ok(axum::Json(response))
}
