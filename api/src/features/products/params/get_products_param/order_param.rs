#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, garde::Validate, utoipa::IntoParams)]
#[serde(rename_all = "kebab-case")]
#[into_params(parameter_in = Query)]
pub struct GetProductsOrder {
    #[serde(default, deserialize_with = "de::deserialize_sort")]
    #[param(
          value_type = String,
          example = "name:asc,price:desc"
    )]
    #[garde(dive)]
    pub sorts: Vec<ProductSort>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ValidProductsOrder {
    pub sorts: Vec<ValidProductSort>,
}

impl GetProductsOrder {
    pub fn validate_into(self) -> Result<ValidProductsOrder, garde::Report> {
        let sorts = self
            .sorts
            .into_iter()
            .map(|s| s.validate_into())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(ValidProductsOrder { sorts })
    }
}

mod de;
mod direction;
mod field;
mod sort;

pub use direction::SortDirection;
pub use field::ProductField;
pub use sort::ProductSort;
pub use sort::ValidProductSort;
