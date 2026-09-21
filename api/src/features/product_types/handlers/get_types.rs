use axum::extract;

use crate::errors;
use crate::features::product_types::models;
use crate::features::product_types::repositories;
use crate::features::product_types::responses;
use crate::shared;
use crate::state;

#[utoipa::path(
    get,
    path = "",
    description = "Get All Product Types",
    tag = "Product Types",
    responses(
        (status = 200, description = "List of Product Types", body = Vec<responses::ProductType>),
        (status = 500, description = "Internal server error", body = shared::responses::ProblemDetails),
    )
)]
pub async fn get_types(
    extract::State(state): extract::State<state::AppState>,
) -> Result<axum::Json<Vec<responses::ProductType>>, errors::ApiError> {
    let product_types = repositories::select_all(&state.pool).await?;

    let response = product_types
        .into_iter()
        .map(models::ProductType::from)
        .map(responses::ProductType::from)
        .collect();

    Ok(axum::Json(response))
}
