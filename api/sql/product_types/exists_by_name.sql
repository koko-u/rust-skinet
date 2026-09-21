SELECT EXISTS (SELECT 1
               FROM "product_types"
               WHERE "name" = $1::varchar) AS "exists!";