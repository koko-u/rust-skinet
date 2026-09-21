use crate::features::product_types::commands;
use crate::features::product_types::rows;
use crate::shared;

pub async fn update(
    tx: &mut shared::Tx<'_>,
    command: &commands::UpdateType,
) -> Result<Option<rows::ProductTypeRow>, sqlx::Error> {
    let commands::UpdateType { id, name } = command;

    sqlx::query_file_as!(
        rows::ProductTypeRow,
        "sql/product_types/update.sql",
        id.into_inner(),
        name,
    )
    .fetch_optional(tx.conn())
    .await
}
