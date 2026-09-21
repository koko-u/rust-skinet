#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FilterProducts {
    pub brands: Option<Vec<String>>,
    pub types: Option<Vec<String>>,
}
