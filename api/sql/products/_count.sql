SELECT COUNT(*) AS "count!"
FROM "products" AS "P"
         INNER JOIN
     "product_types" AS "PT"
     ON
         "P"."product_type_id" = "PT"."id"
         LEFT OUTER JOIN
     "product_brands" AS "PB"
     ON
         "P"."product_brand_id" = "PB"."id"
