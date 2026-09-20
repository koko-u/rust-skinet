use crate::features::health_check::*;
use crate::features::products::openapi::ProductsApi;

mod security_addon;

const MODIFIER: security_addon::SecurityAddon = security_addon::SecurityAddon;

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        ok,
    ),
    nest(
        (path = "/api/products", api = ProductsApi),
    ),
    modifiers(
       &MODIFIER
    )
)]
pub struct ApiDoc;
