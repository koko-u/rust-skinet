WITH "params" ("name") AS (VALUES ($1::varchar)),
     "inserted" AS (
         INSERT INTO "product_types" ("name")
             SELECT "name"
             FROM "params"
             ON CONFLICT ("name") DO NOTHING
             RETURNING "id", "name"),
     "united" AS (SELECT "PT"."id", "PT"."name"
                  FROM "product_types" AS "PT"
                           INNER JOIN
                       "params" AS "P"
                       ON
                           "PT".name = "P"."name"
                  UNION
                  SELECT "id", "name"
                  FROM "inserted")
SELECT "id" AS "id!", "name" AS "name!"
FROM "united";