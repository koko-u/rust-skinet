use std::str::FromStr;

#[derive(
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Default,
    derive_more::Display,
    serde::Deserialize,
    utoipa::ToSchema,
)]
pub enum SortDirection {
    #[default]
    #[display("asc")]
    #[serde(rename = "asc")]
    Ascending,
    #[display("desc")]
    #[serde(rename = "desc")]
    Descending,
}

impl FromStr for SortDirection {
    type Err = derive_more::FromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "asc" => Ok(SortDirection::Ascending),
            "desc" => Ok(SortDirection::Descending),
            _ => Err(derive_more::FromStrError::new("Invalid direction")),
        }
    }
}
