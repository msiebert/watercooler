-- Create the initial users table

CREATE TABLE users (
    id SERIAL NOT NULL,
    email VARCHAR NOT NULL UNIQUE,
    first VARCHAR NOT NULL,
    last VARCHAR NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted TIMESTAMP,
    password VARCHAR NOT NULL,
    owner BOOLEAN NOT NULL,
    PRIMARY KEY(id)
);

CREATE INDEX user_email_index ON users (email);
