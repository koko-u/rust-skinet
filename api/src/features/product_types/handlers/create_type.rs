use axum::extract;

use crate::errors;
use crate::features::product_types::models;
use crate::features::product_types::repositories;
use crate::features::product_types::requests;
use crate::features::product_types::responses;
use crate::shared;
use crate::state;

#[utoipa::path(
    post,
    path = "",
    request_body = requests::CreateType,
    tag = "Product Types",
    responses(
        (status = 201, description = "Created new Product", body = responses::ProductType),
        (status = 400, description = "Validation Error", body = shared::responses::ProblemDetails),
        (status = 500, description = "Internal Server Error", body = shared::responses::ProblemDetails)
    )
)]
pub async fn create_type(
    extract::State(state): extract::State<state::AppState>,
    extract::Json(request): extract::Json<requests::CreateType>,
) -> Result<shared::responses::Created<responses::ProductType>, errors::ApiError> {
    // request validation ( validation errors goto errors::ApiError::Validation(...)
    let command = request.validate_into(&state.pool).await?;

    let created_type: models::ProductType = shared::transaction(&state.pool, async |tx| {
        // create brand
        let created_row = repositories::insert(tx, &command.name).await?;

        match created_row {
            Some(row) => Ok(row.into()),
            None => {
                let mut report = garde::Report::new();
                report.append(
                    garde::Path::empty(),
                    garde::Error::new("Cannot create product type, it may conflict the name"),
                );
                Err(errors::ApiError::Validation(report))
            }
        }
    })
    .await?;

    let location = format!("/api/types/{}", created_type.id);
    Ok(shared::responses::created(location, created_type.into()))
}
