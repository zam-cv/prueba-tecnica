-- Your SQL goes here
CREATE TABLE "users"(
	"id" SERIAL PRIMARY KEY,
	"username" VARCHAR(20) NOT NULL,
	"email" VARCHAR(255) NOT NULL,
	"password" VARCHAR(150) NOT NULL
);

CREATE TABLE "rooms"(
	"id" SERIAL PRIMARY KEY,
	"title" VARCHAR(20) NOT NULL,
	"description" VARCHAR(600) NOT NULL,
	"front_image" VARCHAR(100) NOT NULL,
	"image" VARCHAR(100) NOT NULL,
	"duration" INT4 NOT NULL,
	"answer" VARCHAR(200) NOT NULL
);

