use sqlx_qb_helper::Where;

use crate::features::products::params;

pub async fn count_by_filter(
    pool: &sqlx::PgPool,
    filter: &params::ValidProductsFilter,
) -> Result<i64, sqlx::Error> {
    let mut conn = pool.acquire().await?;

    let params::ValidProductsFilter { brands, types } = filter;
    let product_brand_names = brands.as_ref().map(|b| b.as_slice());
    let product_type_names = types.as_ref().map(|t| t.as_slice());

    let sql_fragment = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/sql/products/_count.sql"));
    let mut query_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new(sql_fragment);
    // where clause
    {
        let mut where_clause = Where::new(&mut query_builder);
        // product_type names
        // WHERE (array ['Audio'] IS NULL OR "PT"."name" = ANY (array ['Audio']) )
        where_clause.and_opt(product_type_names, |qb, names| {
            qb.push(r#" "PT"."name" = ANY( "#).push_bind(names).push(")");
        });
        where_clause.and_opt(product_brand_names, |qb, names| {
            qb.push(r#" "PB"."name" = ANY( "#)
                .push_bind(names)
                .push(r#" ) OR "P"."product_brand_id" IS NULL "#);
        });
    }

    query_builder
        .build_query_scalar::<i64>()
        .fetch_one(&mut *conn)
        .await
}
