use crate::features::product_types::models;
use crate::features::product_types::rows;
use crate::shared;

pub async fn delete(
    tx: &mut shared::Tx<'_>,
    id: models::ProductTypeId,
) -> Result<Option<rows::ProductTypeRow>, sqlx::Error> {
    sqlx::query_file_as!(
        rows::ProductTypeRow,
        "sql/product_types/delete.sql",
        id.into_inner()
    )
    .fetch_optional(tx.conn())
    .await
}
