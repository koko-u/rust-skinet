use crate::features::product_brands::models as pb_models;
use crate::features::product_types::models as pt_models;
use crate::features::products::models;

#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct ProductRow {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub price: rust_decimal::Decimal,
    pub picture_url: Option<String>,
    pub product_type_id: uuid::Uuid,
    pub product_type_name: String,
    pub product_brand_id: Option<uuid::Uuid>,
    pub product_brand_name: Option<String>,
    pub quantity_in_stock: i32,
}

impl From<ProductRow> for models::Product {
    fn from(value: ProductRow) -> Self {
        let product_brand = match (value.product_brand_id, value.product_brand_name) {
            (Some(id), Some(name)) => Some(pb_models::ProductBrand { id: id.into(), name }),
            _ => None,
        };
        Self {
            id: value.id.into(),
            name: value.name,
            description: value.description,
            price: value.price,
            picture_url: value.picture_url,
            product_type: pt_models::ProductType {
                id: value.product_type_id.into(),
                name: value.product_type_name,
            },
            product_brand,
            quantity_in_stock: value.quantity_in_stock,
        }
    }
}
