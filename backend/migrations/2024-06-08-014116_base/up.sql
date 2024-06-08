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
	"example" VARCHAR(100) NOT NULL,
	"answer" VARCHAR(200) NOT NULL
);

CREATE TABLE "solving_times"(
	"room_id" INT4 NOT NULL,
	"user_id" INT4 NOT NULL,
	"time" INT4 NOT NULL,
	PRIMARY KEY("room_id", "user_id"),
	FOREIGN KEY ("room_id") REFERENCES "rooms"("id"),
	FOREIGN KEY ("user_id") REFERENCES "users"("id")
);
