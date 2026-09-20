use axum::extract;

use crate::errors;
use crate::features::product_brands::repositories as pb_repositories;
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
    let command = request.validate_into(id, &state.pool).await?;

    let updated_product = shared::transaction(&state.pool, async |tx| {
        // create product type if not exists
        let product_type = pt_repositories::select_or_insert(tx, &command.product_type).await?;
        // create product brand if not exists
        let product_brand = if let Some(product_brand) = &command.product_brand {
            Some(pb_repositories::select_or_insert(tx, product_brand).await?)
        } else {
            None
        };

        // update product
        let product_brand_id = product_brand.map(|b| b.id.into());
        let row = repositories::update(tx, &command, product_type.id.into(), product_brand_id).await?;

        match row {
            Some(row) => Ok(models::Product::from(row)),
            None => {
                let mut report = garde::Report::new();
                report.append(
                    garde::Path::empty(),
                    garde::Error::new("Cannot update product, it may conflict the name"),
                );
                Err(errors::ApiError::Validation(report))
            }
        }
    })
    .await?;

    Ok(axum::Json(updated_product.into()))
}
