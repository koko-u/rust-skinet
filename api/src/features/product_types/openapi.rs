use crate::features::product_types::handlers::*;
use crate::features::product_types::models::*;

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(get_types, get_type_by_id, create_type, update_type, delete_type,),
    components(schemas(ProductTypeId,))
)]
pub struct TypesApi;
