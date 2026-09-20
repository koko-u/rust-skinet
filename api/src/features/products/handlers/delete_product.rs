use axum::extract;
use axum::http;

use crate::errors;
use crate::features::products::models;
use crate::features::products::repositories;
use crate::shared;
use crate::state;

#[utoipa::path(
    delete,
    path = "/{id}",
    params(
        ("id" = models::ProductId, Path, description = "Product Id")
    ),
    description = "Delete product by Id",
    tag = "Products",
    responses(
        (status = 204, description = "No Content"),
        (status = 404, description = "the Product not found", body = shared::responses::ProblemDetails),
        (status = 500, description = "Internal server error", body = shared::responses::ProblemDetails),
    )
)]
pub async fn delete_product(
    extract::Path(id): extract::Path<models::ProductId>,
    extract::State(state): extract::State<state::AppState>,
) -> Result<http::StatusCode, errors::ApiError> {
    let deleted_product = shared::transaction(&state.pool, async |tx| {
        let row = repositories::delete(tx, id).await?;

        match row {
            Some(row) => Ok(models::Product::from(row)),
            None => {
                let mut report = garde::Report::new();
                report.append(
                    garde::Path::new("id"),
                    garde::Error::new(format!("Product id='{id}' not found")),
                );
                Err(errors::ApiError::Validation(report))
            }
        }
    })
    .await?;

    Ok(http::StatusCode::NO_CONTENT)
}
