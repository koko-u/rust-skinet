use crate::features::products::models;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UpdateProduct {
    pub id: models::ProductId,
    pub name: String,
    pub description: Option<String>,
    pub price: rust_decimal::Decimal,
    pub picture_url: Option<url::Url>,
    pub product_type: String,
    pub brand: Option<String>,
    pub quantity_in_stock: i32,
}
