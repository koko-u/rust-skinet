use crate::features::products::models;
use crate::features::products::rows;
use crate::shared;

pub async fn delete<'a>(
    tx: &'a mut shared::Tx<'_>,
    id: models::ProductId,
) -> Result<Option<rows::ProductRow>, sqlx::Error> {
    sqlx::query_file_as!(rows::ProductRow, "sql/products/delete.sql", id.into_inner())
        .fetch_optional(tx.conn())
        .await
}
