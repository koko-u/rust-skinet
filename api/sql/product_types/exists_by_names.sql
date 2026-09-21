WITH "params" AS (SELECT "name"
                  FROM unnest($1::varchar[]) as x("name"))
SELECT "P"."name"            AS "name!",
       "PT"."id" IS NOT NULL AS "exists!"
FROM "params" AS "P"
         LEFT OUTER JOIN
     "product_types" AS "PT"
     ON
         "P"."name" = "PT"."name";