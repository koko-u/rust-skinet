SELECT EXISTS (SELECT 1
               FROM "product_types"
               WHERE "name" = $2::varchar
                 AND "id" <> $1::uuid) AS "exists!";