use crate::features::product_types::models;

#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct ProductTypeRow {
    pub id: uuid::Uuid,
    pub name: String,
}

impl From<ProductTypeRow> for models::ProductType {
    fn from(value: ProductTypeRow) -> Self {
        Self {
            id: value.id.into(),
            name: value.name,
        }
    }
}
