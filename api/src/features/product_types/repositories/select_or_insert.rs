use crate::features::product_types::rows;
use crate::shared;

pub async fn select_or_insert(
    tx: &mut shared::Tx<'_>,
    name: &str,
) -> Result<rows::ProductTypeRow, sqlx::Error> {
    sqlx::query_file_as!(
        rows::ProductTypeRow,
        "sql/product_types/select_or_insert.sql",
        name
    )
    .fetch_one(tx.conn())
    .await
}
