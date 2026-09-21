use crate::features::product_types::models;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UpdateType {
    pub id: models::ProductTypeId,
    pub name: String,
}
