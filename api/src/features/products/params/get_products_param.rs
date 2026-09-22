#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize)]
pub struct GetProductParam {
    #[serde(flatten)]
    pub filter: GetProductsFilter,
    #[serde(flatten)]
    pub order: GetProductsOrder,
    #[serde(flatten)]
    pub paging: PagingParam,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ValidGetProductsParam {
    pub filter: ValidProductsFilter,
    pub order: ValidProductsOrder,
    pub paging: ValidPagingParam,
}

impl GetProductParam {
    pub async fn validate_into(self, pool: &sqlx::PgPool) -> Result<ValidGetProductsParam, errors::ApiError> {
        let filter_result = self.filter.validate_into(pool).await;
        let order_result = self.order.validate_into();
        let paging_result = self.paging.validate_into();

        match filter_result {
            Ok(filter) => match (order_result, paging_result) {
                (Ok(order), Ok(paging)) => Ok(ValidGetProductsParam {
                    filter,
                    order,
                    paging,
                }),
                (r2, r3) => {
                    let report = merge!(r2, r3);
                    Err(errors::ApiError::Validation(report))
                }
            },
            Err(errors::ValidationError::Validation(report)) => {
                let r1: Result<(), garde::Report> = Err(report);
                let report = merge!(r1, order_result, paging_result);
                Err(errors::ApiError::Validation(report))
            }
            Err(errors::ValidationError::Database(error)) => Err(errors::ApiError::Database(error)),
        }
    }
}

mod filter_param;
mod order_param;
mod paging_param;

pub use filter_param::GetProductsFilter;
pub use filter_param::ValidProductsFilter;
pub use order_param::GetProductsOrder;
pub use order_param::ProductField;
pub use order_param::ProductSort;
pub use order_param::SortDirection;
pub use order_param::ValidProductSort;
pub use order_param::ValidProductsOrder;
pub use paging_param::PagingParam;
pub use paging_param::ValidPagingParam;

use crate::errors;
use crate::shared::macros::merge;
