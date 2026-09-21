use axum::extract;

use crate::errors;
use crate::features::product_types::models;
use crate::features::product_types::repositories;
use crate::features::product_types::requests;
use crate::features::product_types::responses;
use crate::shared;
use crate::state;

#[utoipa::path(
    put,
    path = "/{id}",
    params(
        ("id" = models::ProductTypeId, Path, description = "Product type Id")
    ),
    request_body = requests::UpdateType,
    tag = "Product Types",
    responses(
        (status = 200, description = "the Updated Product type", body = responses::ProductType),
        (status = 400, description = "Validation Error", body = shared::responses::ProblemDetails),
        (status = 500, description = "Internal Server Error", body = shared::responses::ProblemDetails)
    )
)]
pub async fn update_type(
    extract::Path(id): extract::Path<models::ProductTypeId>,
    extract::State(state): extract::State<state::AppState>,
    extract::Json(request): extract::Json<requests::UpdateType>,
) -> Result<axum::Json<responses::ProductType>, errors::ApiError> {
    // validate request
    let command = request.validate_into(id, &state.pool).await?;

    let updated_type = shared::transaction(&state.pool, async |tx| {
        // update product type
        let row = repositories::update(tx, &command).await?;

        match row {
            Some(row) => Ok(models::ProductType::from(row)),
            None => {
                let mut report = garde::Report::new();
                report.append(
                    garde::Path::empty(),
                    garde::Error::new("Cannot update product type, it may conflict the name"),
                );
                Err(errors::ApiError::Validation(report))
            }
        }
    })
    .await?;

    Ok(axum::Json(updated_type.into()))
}
