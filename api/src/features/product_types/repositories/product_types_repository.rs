use crate::features::product_types::rows;

#[derive(Debug)]
pub struct ProductTypesRepository<'c>(&'c mut sqlx::PgConnection);

impl<'c> ProductTypesRepository<'c> {
    pub fn new(connection: &'c mut sqlx::PgConnection) -> Self {
        Self(connection)
    }

    pub async fn insert(self, name: &str) -> Result<Option<rows::ProductTypeRow>, sqlx::Error> {
        sqlx::query_file_as!(rows::ProductTypeRow, "sql/product_types/insert.sql", name)
            .fetch_optional(self.0.as_mut())
            .await
    }

    pub async fn select_or_insert(self, name: &str) -> Result<rows::ProductTypeRow, sqlx::Error> {
        sqlx::query_file_as!(
            rows::ProductTypeRow,
            "sql/product_types/select_or_insert.sql",
            name
        )
        .fetch_one(self.0.as_mut())
        .await
    }
}
