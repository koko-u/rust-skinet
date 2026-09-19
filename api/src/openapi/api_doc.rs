use crate::features::health_check::*;
use crate::features::products::handlers::*;

mod security_addon;

const MODIFIER: security_addon::SecurityAddon = security_addon::SecurityAddon;

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        ok,
        get_products,
        get_product_by_id,
    ),
    modifiers(
       &MODIFIER
    )
)]
pub struct ApiDoc;
