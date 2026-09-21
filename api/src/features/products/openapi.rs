use crate::features::products::handlers::*;
use crate::features::products::models::*;

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        get_products,
        get_product_by_id,
        create_product,
        update_product,
        delete_product,
    ),
    components(schemas(ProductId,))
)]
pub struct ProductsApi;
