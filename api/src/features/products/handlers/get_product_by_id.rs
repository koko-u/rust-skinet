use axum::extract;

use crate::errors;
use crate::features::products::models;
use crate::features::products::products_repository::ProductsRepository;
use crate::features::products::responses;
use crate::shared;
use crate::state;

#[utoipa::path(
    get,
    path = "/api/products/{id}",
    params(
        ("id" = models::ProductId, Path, description = "Product Id")
    ),
    description = "Get One Product by Id",
    tag = "Products",
    responses(
        (status = 200, description = "Product for supplied id", body = responses::Product),
        (status = 404, description = "the Product is not found", body = shared::responses::ProblemDetails),
        (status = 500, description = "Internal server error", body = shared::responses::ProblemDetails),
    )
)]
pub async fn get_product_by_id(
    extract::Path(id): extract::Path<models::ProductId>,
    extract::State(state): extract::State<state::AppState>,
) -> Result<axum::Json<responses::Product>, errors::ApiError> {
    let mut conn = state.pool.acquire().await?;
    let repo = ProductsRepository::from(conn.as_mut());
    let product = repo.select_by_id(id).await?;

    match product {
        Some(product) => {
            let product: models::Product = product.into();
            let product: responses::Product = product.into();
            Ok(axum::Json(product))
        }
        None => Err(errors::ApiError::NotFound {
            message: format!("Product with Id={id} not found"),
        }),
    }
}
