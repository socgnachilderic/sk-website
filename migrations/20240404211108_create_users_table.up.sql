-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
    user_id uuid DEFAULT gen_random_uuid(),
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL,
    email VARCHAR(50) UNIQUE,
    phone_number VARCHAR(20) UNIQUE,
    jobs VARCHAR(50) NOT NULL,
    birthdate DATE NOT NULL,
    country VARCHAR(50) NOT NULL,
    city VARCHAR(50) NOT NULL,
    is_superuser BOOLEAN NOT NULL DEFAULT FALSE,

    PRIMARY KEY (user_id)
);
