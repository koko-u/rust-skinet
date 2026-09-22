use crate::errors;
use crate::features::products::commands;
use crate::features::products::repositories;
use crate::shared::macros::merge;
use crate::shared::validators;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, garde::Validate, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateProduct {
    #[garde(required, length(max = 255))]
    #[schema(required, max_length = 255)]
    pub name: Option<String>,

    #[garde(length(max = 1000))]
    #[schema(max_length = 1000)]
    pub description: Option<String>,

    #[garde(required, custom(validators::opt_decimal_places(2)))]
    #[schema(required, multiple_of = 0.01)]
    pub price: Option<rust_decimal::Decimal>,

    #[garde(length(max = 2048))]
    #[schema(max_length = 2048)]
    pub picture_url: Option<String>,

    #[garde(required, length(max = 255))]
    #[schema(required, max_length = 255)]
    pub product_type: Option<String>,

    #[garde(length(max = 255))]
    #[schema(max_length = 255)]
    pub product_brand: Option<String>,

    #[garde(required, range(min = 0))]
    #[schema(required, minimum = 0)]
    pub quantity_in_stock: Option<i32>,
}

impl CreateProduct {
    pub async fn validate_into(
        self,
        pool: &sqlx::PgPool,
    ) -> Result<commands::CreateProduct, errors::ValidationError> {
        use garde::Validate as _;
        let garde_result = self.validate();

        // name already exists error
        let mut name_result: Result<(), garde::Report> = Ok(());
        if let Some(name) = &self.name {
            let mut conn = pool.acquire().await?;
            let exists = repositories::exists_by_name(pool, name).await?;
            if exists {
                let mut error = garde::Report::new();
                error.append(
                    garde::Path::new("name"),
                    garde::Error::new(format!("product name={name} is already exists")),
                );

                name_result = Err(error);
            }
        }

        match (garde_result, name_result) {
            (Ok(_), Ok(_)) => Ok(commands::CreateProduct {
                name: self.name.expect("name is required"),
                description: self.description,
                price: self.price.expect("price is required"),
                picture_url: self.picture_url,
                product_type: self.product_type.expect("product_type is required"),
                product_brand: self.product_brand,
                quantity_in_stock: self.quantity_in_stock.expect("quantity_in_stock is required"),
            }),
            (result1, result2) => {
                let report = merge!(result1, result2);
                Err(errors::ValidationError::Validation(report))
            }
        }
    }
}
