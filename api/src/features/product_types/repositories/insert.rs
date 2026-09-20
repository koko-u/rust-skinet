use crate::features::product_types::rows;
use crate::shared;

pub async fn insert(
    tx: &mut shared::Tx<'_>,
    name: &str,
) -> Result<Option<rows::ProductTypeRow>, sqlx::Error> {
    sqlx::query_file_as!(rows::ProductTypeRow, "sql/product_types/insert.sql", name)
        .fetch_optional(tx.conn())
        .await
}
