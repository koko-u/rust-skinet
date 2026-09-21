WITH "selected_types" AS (SELECT "id", "name"
                          FROM "product_types"
                          WHERE (
                                    $1::varchar[] IS NULL
                                        OR
                                    "name" = ANY ($1::varchar[])
                                    )),
     "selected_brands" AS (SELECT "id", "name"
                           FROM "product_brands"
                           WHERE (
                                     $2::varchar[] IS NULL
                                         OR
                                     "name" = ANY ($2::varchar[])
                                     )),
     "filtered" AS (SELECT "P"."id",
                           "P"."name",
                           "P"."description",
                           "P"."price",
                           "P"."picture_url",
                           "P"."product_type_id",
                           "PT"."name" AS "product_type_name",
                           "P"."product_brand_id",
                           "PB"."name" AS "product_brand_name",
                           "P"."quantity_in_stock"
                    FROM "products" AS "P"
                             INNER JOIN
                         "selected_types" AS "PT"
                         ON
                             "P"."product_type_id" = "PT"."id"
                             INNER JOIN
                         "selected_brands" AS "PB"
                         ON "P"."product_brand_id" = "PB"."id"),
     "null_brands" AS (SELECT "P"."id",
                              "P"."name",
                              "P"."description",
                              "P"."price",
                              "P"."picture_url",
                              "P"."product_type_id",
                              "PT"."name" AS "product_type_name",
                              "P"."product_brand_id",
                              NULL        AS "product_brand_name",
                              "P"."quantity_in_stock"
                       FROM "products" AS "P"
                                INNER JOIN
                            "selected_types" AS "PT"
                            ON
                                "P"."product_type_id" = "PT"."id"
                       WHERE "P"."product_brand_id" IS NULL),
     "united" AS (SELECT *
                  FROM "filtered"
                  UNION
                  SELECT *
                  FROM "null_brands")
SELECT "id"                 AS "id!",
       "name"               AS "name!",
       "description"        AS "description",
       "price"              AS "price!",
       "picture_url"        AS "picture_url",
       "product_type_id"    AS "product_type_id!",
       "product_type_name"  AS "product_type_name!",
       "product_brand_id"   AS "product_brand_id",
       "product_brand_name" AS "product_brand_name",
       "quantity_in_stock"  AS "quantity_in_stock!"
FROM "united"
ORDER BY "id";