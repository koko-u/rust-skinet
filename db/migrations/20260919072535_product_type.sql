-- migrate:up
CREATE TABLE product_types
(
    "id"         uuid         NOT NULL DEFAULT uuidv7(),
    "name"       varchar(255) NOT NULL,
    "created_at" TIMESTAMPTZ  NOT NULL DEFAULT now(),
    "updated_at" TIMESTAMPTZ  NOT NULL DEFAULT now(),
    CONSTRAINT "product_type_pkey" PRIMARY KEY ("id"),
    CONSTRAINT "product_type_name_unique" UNIQUE ("name")
);

CREATE INDEX "product_type_name_idx_like" ON product_types USING gin ("name" gin_trgm_ops);

CREATE TRIGGER "product_type_update_at_tgr"
    BEFORE UPDATE
    ON product_types
    FOR EACH ROW
EXECUTE PROCEDURE moddatetime("updated_at");

-- migrate:down
DROP TABLE "product_types";
