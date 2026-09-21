use crate::features::product_types::models;

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, utoipa::ToSchema)]
pub struct ProductType {
    pub id: uuid::Uuid,
    pub name: String,
}

impl From<models::ProductType> for ProductType {
    fn from(value: models::ProductType) -> Self {
        Self {
            id: value.id.into_inner(),
            name: value.name,
        }
    }
}
