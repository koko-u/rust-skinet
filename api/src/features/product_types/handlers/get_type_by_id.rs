use axum::extract;

use crate::errors;
use crate::features::product_types::models;
use crate::features::product_types::repositories;
use crate::features::product_types::responses;
use crate::shared;
use crate::state;

#[utoipa::path(
    get,
    path = "/{id}",
    params(
        ("id" = models::ProductTypeId, Path, description = "Product type Id")
    ),
    description = "Get One Product type by Id",
    tag = "Product Types",
    responses(
        (status = 200, description = "Product type for supplied id", body = responses::ProductType),
        (status = 404, description = "the Product is not found", body = shared::responses::ProblemDetails),
        (status = 500, description = "Internal server error", body = shared::responses::ProblemDetails),
    )
)]
pub async fn get_type_by_id(
    extract::Path(id): extract::Path<models::ProductTypeId>,
    extract::State(state): extract::State<state::AppState>,
) -> Result<axum::Json<responses::ProductType>, errors::ApiError> {
    let product_type = repositories::select_by_id(&state.pool, id).await?;

    match product_type {
        Some(product) => {
            let product: models::ProductType = product.into();
            let product: responses::ProductType = product.into();
            Ok(axum::Json(product))
        }
        None => Err(errors::ApiError::NotFound {
            message: format!("Product type with Id={id} not found"),
        }),
    }
}
