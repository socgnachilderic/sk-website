-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
    user_id uuid DEFAULT gen_random_uuid(),
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    phone_number VARCHAR(20) NOT NULL,
    jobs VARCHAR(100) NOT NULL,
    birthdate DATE NOT NULL,
    country VARCHAR(100) NOT NULL,
    city VARCHAR(100) NOT NULL,

    PRIMARY KEY (user_id)
);
