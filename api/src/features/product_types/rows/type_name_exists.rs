#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct TypeNameExists {
    pub name: String,
    pub exists: bool,
}
