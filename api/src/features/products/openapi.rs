use crate::features::products::handlers::*;
#[derive(utoipa::OpenApi)]
#[openapi(paths(
    get_products,
    get_product_by_id,
    create_product,
    update_product,
    delete_product,
))]
pub struct ProductsApi;
