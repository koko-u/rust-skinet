use axum::extract;

use crate::errors;
use crate::features::product_types::repositories as pt_repositories;
use crate::features::products::models;
use crate::features::products::repositories;
use crate::features::products::requests;
use crate::features::products::responses;
use crate::shared;
use crate::state;
#[utoipa::path(
    put,
    path = "/{id}",
    params(
        ("id" = models::ProductId, Path, description = "Product Id")
    ),
    request_body = requests::UpdateProduct,
    tag = "Products",
    responses(
        (status = 200, description = "the Updated Product", body = responses::Product),
        (status = 400, description = "Validation Error", body = shared::responses::ProblemDetails),
        (status = 500, description = "Internal Server Error", body = shared::responses::ProblemDetails)
    )
)]
pub async fn update_product(
    extract::Path(id): extract::Path<models::ProductId>,
    extract::State(state): extract::State<state::AppState>,
    extract::Json(request): extract::Json<requests::UpdateProduct>,
) -> Result<axum::Json<responses::Product>, errors::ApiError> {
    let mut conn = state.pool.acquire().await?;
    let command = request.validate_into(id, &mut conn).await?;

    let updated_product = {
        let mut tx = state.pool.begin().await?;
        let product_types_repo = pt_repositories::ProductTypesRepository::new(tx.as_mut());
        let product_type = product_types_repo.select_or_insert(&command.product_type).await?;

        let product_types_repo = repositories::ProductsRepository::new(tx.as_mut());
        let row = product_types_repo
            .update(&command, product_type.id.into())
            .await?;

        tx.commit().await?;

        match row {
            Some(row) => models::Product::from(row),
            None => {
                let mut report = garde::Report::new();
                report.append(
                    garde::Path::empty(),
                    garde::Error::new("Cannot update product, it may conflict the name"),
                );
                return Err(errors::ApiError::Validation(report));
            }
        }
    };

    Ok(axum::Json(updated_product.into()))
}
