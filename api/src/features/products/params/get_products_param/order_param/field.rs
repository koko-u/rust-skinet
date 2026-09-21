use std::str::FromStr;

#[derive(
    Debug, Copy, Clone, Eq, PartialEq, Hash, derive_more::Display, serde::Deserialize, utoipa::ToSchema,
)]
pub enum ProductField {
    #[display("id")]
    #[serde(rename = "id")]
    Id,

    #[display("name")]
    #[serde(rename = "name")]
    Name,

    #[display("price")]
    #[serde(rename = "price")]
    Price,

    #[display("type")]
    #[serde(rename = "type")]
    Type,

    #[display("brand")]
    #[serde(rename = "brand")]
    Brand,

    #[display("stock")]
    #[serde(rename = "stock")]
    QuantityInStock,
}

impl FromStr for ProductField {
    type Err = derive_more::FromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "id" => Ok(ProductField::Id),
            "name" => Ok(ProductField::Name),
            "price" => Ok(ProductField::Price),
            "type" => Ok(ProductField::Type),
            "brand" => Ok(ProductField::Brand),
            "stock" => Ok(ProductField::QuantityInStock),
            _ => Err(derive_more::FromStrError::new("Invalid field type")),
        }
    }
}
