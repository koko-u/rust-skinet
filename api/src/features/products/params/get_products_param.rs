#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize)]
pub struct GetProductParam {
    #[serde(flatten)]
    pub filter: GetProductsFilter,
    #[serde(flatten)]
    pub order: GetProductsOrder,
}

mod filter_param;
mod order_param;

pub use filter_param::GetProductsFilter;
pub use filter_param::ValidProductsFilter;
pub use order_param::GetProductsOrder;
pub use order_param::ProductField;
pub use order_param::ProductSort;
pub use order_param::SortDirection;pub use order_param::ValidProductSort;
pub use order_param::ValidProductsOrder;

