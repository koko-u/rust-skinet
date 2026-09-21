use crate::features::product_types::handlers::*;

#[derive(utoipa::OpenApi)]
#[openapi(paths(get_types, get_type_by_id, create_type, update_type, delete_type,))]
pub struct TypesApi;
