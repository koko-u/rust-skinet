use sqlx_qb_helper::OrderBy;
use sqlx_qb_helper::Where;

use crate::features::products::params;
use crate::features::products::rows;

pub async fn select_by_filter(
    pool: &sqlx::PgPool,
    filter: &params::ValidProductsFilter,
    order: &params::ValidProductsOrder,
    paging: params::ValidPagingParam,
) -> Result<Vec<rows::ProductRow>, sqlx::Error> {
    let mut conn = pool.acquire().await?;

    let params::ValidProductsFilter { brands, types } = filter;
    let sorts = &order.sorts;
    let product_brand_names = brands.as_ref().map(|b| b.as_slice());
    let product_type_names = types.as_ref().map(|t| t.as_slice());

    let sql_fragment = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/sql/products/_select.sql"));
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
    // order clause
    {
        let mut order_by = OrderBy::new(&mut query_builder);
        for sort in sorts {
            let params::ValidProductSort { field, direction } = sort;

            order_by.push(|qb| match (field, direction) {
                (params::ProductField::Id, params::SortDirection::Ascending) => {
                    qb.push(r#" "P"."id" ASC "#);
                }
                (params::ProductField::Id, params::SortDirection::Descending) => {
                    qb.push(r#" "P"."id" DESC "#);
                }
                (params::ProductField::Name, params::SortDirection::Ascending) => {
                    qb.push(r#" "P"."name" ASC "#);
                }
                (params::ProductField::Name, params::SortDirection::Descending) => {
                    qb.push(r#" "P"."name" DESC "#);
                }
                (params::ProductField::Price, params::SortDirection::Ascending) => {
                    qb.push(r#" "P"."price" ASC "#);
                }
                (params::ProductField::Price, params::SortDirection::Descending) => {
                    qb.push(r#" "P"."price" DESC "#);
                }
                (params::ProductField::Type, params::SortDirection::Ascending) => {
                    qb.push(r#" "PT"."name" ASC "#);
                }
                (params::ProductField::Type, params::SortDirection::Descending) => {
                    qb.push(r#" "PT"."name" DESC "#);
                }
                (params::ProductField::Brand, params::SortDirection::Ascending) => {
                    qb.push(r#" "PB"."name" ASC "#);
                }
                (params::ProductField::Brand, params::SortDirection::Descending) => {
                    qb.push(r#" "PB"."name" DESC "#);
                }
                (params::ProductField::QuantityInStock, params::SortDirection::Ascending) => {
                    qb.push(r#" "P"."quantity_in_stock" ASC "#);
                }
                (params::ProductField::QuantityInStock, params::SortDirection::Descending) => {
                    qb.push(r#" "P"."quantity_in_stock" DESC "#);
                }
            });
        }
    }
    // limit and offset
    query_builder.push(" LIMIT ").push_bind(paging.limit());
    query_builder.push(" OFFSET ").push_bind(paging.offset());

    query_builder
        .build_query_as::<rows::ProductRow>()
        .fetch_all(&mut *conn)
        .await

    // sqlx::query_file_as!(
    //     rows::ProductRow,
    //     "sql/products/select_by_filter.sql",
    //     product_type_names,
    //     product_brand_names
    // )
    // .fetch_all(&mut *conn)
    // .await
}
