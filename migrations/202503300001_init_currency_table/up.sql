-- Create currency table
CREATE TABLE "currency" (
    "id"     INTEGER,
    "rate"   TEXT NOT NULL,
    "symbol" TEXT NOT NULL,
    "name"   TEXT NOT NULL,
    UNIQUE("symbol"),
    PRIMARY KEY("id" AUTOINCREMENT)
);

-- British Pound is always the first/default currency
INSERT INTO "currency" ("rate", "symbol", "name") VALUES ('1', '£', 'British Pound');

-- Recreate item table with currency_id (SQLite cannot add FK via ALTER TABLE)
ALTER TABLE "item" RENAME TO "_item_old";

CREATE TABLE "item" (
    "id"          INTEGER,
    "oldId"       INTEGER,
    "category"    TEXT NOT NULL CHECK(category IN ('bank', 'cardbalance', 'creditcard', 'cash', 'debt', 'misc', 'recurring')),
    "name"        TEXT NOT NULL,
    "day"         INTEGER,
    "amount"      TEXT NOT NULL,
    "cardid"      INTEGER,
    "dbName"      TEXT CHECK(dbName IN ('debit', 'credit')),
    "currency_id" INTEGER REFERENCES "currency"("id"),
    PRIMARY KEY("id" AUTOINCREMENT)
);

-- Copy existing data, defaulting currency_id to 1 (GBP)
INSERT INTO "item" ("id", "oldId", "category", "name", "day", "amount", "cardid", "dbName", "currency_id")
SELECT "id", "oldId", "category", "name", "day", "amount", "cardid", "dbName", 1
FROM "_item_old";

DROP TABLE "_item_old";

-- Migrate old "CUR:rate [name]" items into proper currency rows and update references.
-- Extract the rate string: everything between "CUR:" (pos 5) and the first space, or end of string.
INSERT OR IGNORE INTO "currency" ("rate", "symbol", "name")
SELECT DISTINCT
    CASE
        WHEN INSTR(SUBSTR("name", 5), ' ') > 0
        THEN SUBSTR("name", 5, INSTR(SUBSTR("name", 5), ' ') - 1)
        ELSE SUBSTR("name", 5)
    END,
    'CUR_' || REPLACE(
        CASE
            WHEN INSTR(SUBSTR("name", 5), ' ') > 0
            THEN SUBSTR("name", 5, INSTR(SUBSTR("name", 5), ' ') - 1)
            ELSE SUBSTR("name", 5)
        END,
    '.', '_'),
    'Currency (rate ' ||
        CASE
            WHEN INSTR(SUBSTR("name", 5), ' ') > 0
            THEN SUBSTR("name", 5, INSTR(SUBSTR("name", 5), ' ') - 1)
            ELSE SUBSTR("name", 5)
        END
    || ')'
FROM "item"
WHERE "name" LIKE 'CUR:%';

-- Update items: link to the right currency and strip the "CUR:rate " prefix from name.
UPDATE "item"
SET
    "currency_id" = (
        SELECT "id" FROM "currency"
        WHERE "rate" =
            CASE
                WHEN INSTR(SUBSTR("item"."name", 5), ' ') > 0
                THEN SUBSTR("item"."name", 5, INSTR(SUBSTR("item"."name", 5), ' ') - 1)
                ELSE SUBSTR("item"."name", 5)
            END
        LIMIT 1
    ),
    "name" = CASE
        WHEN INSTR(SUBSTR("item"."name", 5), ' ') > 0
        THEN TRIM(SUBSTR("item"."name", 5 + INSTR(SUBSTR("item"."name", 5), ' ')))
        ELSE 'Unnamed'
    END
WHERE "item"."name" LIKE 'CUR:%';

-- Trigger enforces referential integrity at the schema level:
-- prevents deleting a currency that is linked to one or more items.
CREATE TRIGGER prevent_currency_delete
BEFORE DELETE ON "currency"
FOR EACH ROW
BEGIN
    SELECT RAISE(ABORT, 'Cannot delete currency: items are linked to it')
    WHERE EXISTS (SELECT 1 FROM "item" WHERE "currency_id" = OLD.id);
END;
