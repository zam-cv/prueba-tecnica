-- Your SQL goes here

CREATE TABLE "rooms"(
	"id" SERIAL PRIMARY KEY,
	"title" VARCHAR(20) NOT NULL,
	"description" VARCHAR(600) NOT NULL,
	"front_image" VARCHAR(100) NOT NULL,
	"image" VARCHAR(100) NOT NULL,
	"duration" INT4 NOT NULL
);

