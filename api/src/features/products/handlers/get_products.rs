use axum::extract;

use crate::errors;
use crate::features::products::models;
use crate::features::products::query_params;
use crate::features::products::repositories;
use crate::features::products::responses;
use crate::shared;
use crate::state;

#[utoipa::path(
    get,
    path = "",
    params(query_params::GetProductsQuery),
    description = "Get All Products",
    tag = "Products",
    responses(
        (status = 200, description = "List of Products", body = Vec<responses::Product>),
        (status = 500, description = "Internal server error", body = shared::responses::ProblemDetails),
    )
)]
pub async fn get_products(
    extract::State(state): extract::State<state::AppState>,
    extract::Query(params): extract::Query<query_params::GetProductsQuery>,
) -> Result<axum::Json<Vec<responses::Product>>, errors::ApiError> {
    let filter = params.validate_into(&state.pool).await?;
    let products = repositories::select_by_filter(&state.pool, &filter).await?;

    let response = products
        .into_iter()
        .map(models::Product::from)
        .map(responses::Product::from)
        .collect();

    Ok(axum::Json(response))
}
