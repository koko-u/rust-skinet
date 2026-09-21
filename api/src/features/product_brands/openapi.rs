use crate::features::product_brands::handlers::*;
use crate::features::product_brands::models::*;

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(get_brands, get_brand_by_id, create_brand, update_brand, delete_brand,),
    components(schemas(ProductBrandId,))
)]
pub struct BrandsApi;
