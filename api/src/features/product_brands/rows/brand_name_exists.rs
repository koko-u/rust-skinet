#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct BrandNameExists {
    pub name: String,
    pub exists: bool,
}
