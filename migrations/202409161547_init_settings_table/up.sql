CREATE TABLE "settings" (
	"id"	INTEGER,
	"name"	TEXT NOT NULL,
	"value"	TEXT NOT NULL,
	UNIQUE("name")
	PRIMARY KEY("id" AUTOINCREMENT)
);