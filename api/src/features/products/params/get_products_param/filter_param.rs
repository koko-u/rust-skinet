use crate::errors;
use crate::features::product_brands::repositories as pb_repositories;
use crate::features::product_types::repositories as pt_repositories;
use crate::shared::macros::merge;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, garde::Validate, utoipa::IntoParams)]
#[serde(rename_all = "kebab-case")]
#[into_params(parameter_in = Query)]
pub struct GetProductsFilter {
    #[serde(default, deserialize_with = "de::comma_separated")]
    #[param(style = Form, explode = false)]
    #[garde(inner(length(max = 10), inner(length(max = 255))))]
    pub brands: Option<Vec<String>>,

    #[serde(default, deserialize_with = "de::comma_separated")]
    #[param(style = Form, explode = false)]
    #[garde(inner(length(max = 10), inner(length(max = 255))))]
    pub types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ValidProductsFilter {
    pub brands: Option<Vec<String>>,
    pub types: Option<Vec<String>>,
}

impl GetProductsFilter {
    pub async fn validate_into(
        self,
        pool: &sqlx::PgPool,
    ) -> Result<ValidProductsFilter, errors::ValidationError> {
        use garde::Validate as _;
        let garde_result = self.validate();

        let mut brands_result = Ok(());
        if let Some(brands) = &self.brands {
            let exist_rows = pb_repositories::exists_by_names(pool, brands).await?;
            let non_exist_names = exist_rows
                .into_iter()
                .filter(|row| !row.exists)
                .map(|row| row.name)
                .collect::<Vec<_>>();
            if !non_exist_names.is_empty() {
                let mut report = garde::Report::new();
                report.append(
                    garde::Path::new("brands"),
                    garde::Error::new(format!(
                        "{} is not a valid brand names",
                        non_exist_names.join(",")
                    )),
                );
                brands_result = Err(report);
            }
        }

        let mut types_result = Ok(());
        if let Some(types) = &self.types {
            let exist_rows = pt_repositories::exists_by_names(pool, types).await?;
            let non_exist_names = exist_rows
                .into_iter()
                .filter(|row| !row.exists)
                .map(|row| row.name)
                .collect::<Vec<_>>();
            if !non_exist_names.is_empty() {
                let mut report = garde::Report::new();
                report.append(
                    garde::Path::new("types"),
                    garde::Error::new(format!("{} is not a valid type names", non_exist_names.join(","))),
                );
                types_result = Err(report);
            }
        }

        match (garde_result, brands_result, types_result) {
            (Ok(()), Ok(()), Ok(())) => Ok(ValidProductsFilter {
                brands: self.brands,
                types: self.types,
            }),
            (r1, r2, r3) => {
                let report = merge!(r1, r2, r3);
                Err(errors::ValidationError::Validation(report))
            }
        }
    }
}

mod de;
