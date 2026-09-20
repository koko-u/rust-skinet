use crate::features::products::models;

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, utoipa::ToSchema)]
pub struct Product {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub price: rust_decimal::Decimal,
    pub picture_url: Option<String>,
    pub product_type: String,
    pub brand: Option<String>,
    pub quantity_in_stock: i32,
}

impl From<models::Product> for Product {
    fn from(value: models::Product) -> Self {
        Self {
            id: value.id.into_inner(),
            name: value.name,
            description: value.description,
            price: value.price,
            picture_url: value.picture_url.map(String::from),
            product_type: value.product_type.name,
            brand: value.brand,
            quantity_in_stock: value.quantity_in_stock,
        }
    }
}
