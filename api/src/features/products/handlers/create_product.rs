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
    post,
    path = "",
    request_body = requests::CreateProduct,
    tag = "Products",
    responses(
        (status = 201, description = "Created new Product", body = responses::Product),
        (status = 400, description = "Validation Error", body = shared::responses::ProblemDetails),
        (status = 500, description = "Internal Server Error", body = shared::responses::ProblemDetails)
    )
)]
pub async fn create_product(
    extract::State(state): extract::State<state::AppState>,
    extract::Json(request): extract::Json<requests::CreateProduct>,
) -> Result<shared::responses::Created<responses::Product>, errors::ApiError> {
    let command = request.validate_into(&state.pool).await?;

    let created_product: models::Product = shared::transaction(&state.pool, async |tx| {
        // create product type if not exists
        let product_type = pt_repositories::select_or_insert(tx, &command.product_type).await?;

        // create product
        let created_row = repositories::insert(tx, &command, product_type.id.into()).await?;

        match created_row {
            Some(row) => Ok(row.into()),
            None => {
                let mut report = garde::Report::new();
                report.append(
                    garde::Path::empty(),
                    garde::Error::new("Cannot create product, it may conflict the name"),
                );
                Err(errors::ApiError::Validation(report))
            }
        }
    })
    .await?;

    let location = format!("/api/products/{}", created_product.id);
    Ok(shared::responses::created(location, created_product.into()))
}
