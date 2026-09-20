use better_default::Default as BetterDefault;
use into_inner::IntoInner;

use crate::features::product_types::models as pt_models;

#[derive(
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Hash,
    BetterDefault,
    derive_more::Display,
    derive_more::From,
    derive_more::FromStr,
    into_inner::IntoInner,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[display("{}", _0)]
#[serde(transparent)]
#[schema(value_type = uuid::Uuid)]
pub struct ProductId(#[default(uuid::Uuid::now_v7())] uuid::Uuid);

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Product {
    pub id: ProductId,
    pub name: String,
    pub description: Option<String>,
    pub price: rust_decimal::Decimal,
    pub picture_url: Option<String>,
    pub product_type: pt_models::ProductType,
    pub brand: Option<String>,
    pub quantity_in_stock: i32,
}
