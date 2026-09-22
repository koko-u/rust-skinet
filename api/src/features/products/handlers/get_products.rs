use axum::extract;

use crate::errors;
use crate::features::products::models;
use crate::features::products::params;
use crate::features::products::repositories;
use crate::features::products::responses;
use crate::shared;
use crate::state;

#[utoipa::path(
    get,
    path = "",
    params(
        params::GetProductsFilter,
        params::GetProductsOrder,
        params::PagingParam,
    ),
    description = "Get All Products",
    tag = "Products",
    responses(
        (status = 200, description = "List of Products", body = shared::responses::WithPagination<responses::Product>),
        (status = 500, description = "Internal server error", body = shared::responses::ProblemDetails),
    )
)]
pub async fn get_products(
    extract::State(state): extract::State<state::AppState>,
    extract::Query(params): extract::Query<params::GetProductParam>,
) -> Result<axum::Json<shared::responses::WithPagination<responses::Product>>, errors::ApiError> {
    // validate params
    let params::ValidGetProductsParam {
        filter,
        order,
        paging,
    } = params.validate_into(&state.pool).await?;

    let products = repositories::select_by_filter(&state.pool, &filter, &order, paging).await?;

    let count = repositories::count_by_filter(&state.pool, &filter).await?;
    let total = count as u32;
    let pages = total.div_ceil(paging.page_size.into_inner());

    let products = products
        .into_iter()
        .map(models::Product::from)
        .map(responses::Product::from)
        .collect();
    let response = shared::responses::WithPagination {
        current_page: paging.current_page,
        page_size: paging.page_size,
        total,
        pages,
        items: products,
    };

    Ok(axum::Json(response))
}
