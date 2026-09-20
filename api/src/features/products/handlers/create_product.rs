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
    let mut conn = state.pool.acquire().await?;
    let command = request.validate_into(&mut conn).await?;

    let created_product = {
        let mut tx = state.pool.begin().await?;
        let product_type_repo = pt_repositories::ProductTypesRepository::new(tx.as_mut());
        // create product_type (if already exists, return None)
        let product_type = product_type_repo.select_or_insert(&command.name).await?;

        // create product
        let products_repo = repositories::ProductsRepository::new(tx.as_mut());
        let created_row = products_repo.insert(&command, product_type.id.into()).await?;

        let product: models::Product = match created_row {
            Some(row) => row.into(),
            None => {
                let mut report = garde::Report::new();
                report.append(
                    garde::Path::empty(),
                    garde::Error::new("Cannot create product, it may conflict the name"),
                );
                return Err(errors::ApiError::Validation(report));
            }
        };

        tx.commit().await?;

        product
    };

    let location = format!("/api/products/{}", created_product.id);
    Ok(shared::responses::created(location, created_product.into()))
}
