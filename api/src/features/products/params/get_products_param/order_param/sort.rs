use super::direction;
use super::field;
use crate::shared::validators;

#[derive(Debug, Clone, Eq, PartialEq, garde::Validate, utoipa::ToSchema)]
pub struct ProductSort {
    #[garde(custom(validators::deserializable_as::<field::ProductField>))]
    pub field: String,
    #[garde(custom(validators::deserializable_as::<direction::SortDirection>))]
    pub direction: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ValidProductSort {
    pub field: field::ProductField,
    pub direction: direction::SortDirection,
}

impl ProductSort {
    pub fn validate_into(self) -> Result<ValidProductSort, garde::Report> {
        let field = serde_plain::from_str::<field::ProductField>(&self.field);
        let direction = serde_plain::from_str::<direction::SortDirection>(&self.direction);

        match (field, direction) {
            (Ok(field), Ok(direction)) => Ok(ValidProductSort { field, direction }),
            (field_r, direction_r) => {
                let mut report = garde::Report::new();
                if let Err(e) = field_r {
                    report.append(garde::Path::new("field"), garde::Error::new(e.to_string()));
                }
                if let Err(e) = direction_r {
                    report.append(garde::Path::new("direction"), garde::Error::new(e.to_string()));
                }

                Err(report)
            }
        }
    }
}
