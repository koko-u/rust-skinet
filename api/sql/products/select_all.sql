SELECT "P"."id",
       "P"."name",
       "P"."description",
       "P"."price",
       "P"."picture_url",
       "P"."product_type_id",
       "PT"."name" AS "product_type_name!",
       "P"."brand",
       "P"."quantity_in_stock"
FROM "products" AS "P"
         INNER JOIN
     "product_types" AS "PT"
     ON
         "P"."product_type_id" = "PT"."id"
ORDER BY "P"."id";