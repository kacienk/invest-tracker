CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE investment_users (
    id UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE CHECK (email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}$'),
    password VARCHAR(255) NOT NULL,
    salt VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE investment_groups (
    id UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
    group_name VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    deleted BOOLEAN NOT NULL DEFAULT FALSE,
    owner_id UUID NOT NULL,
    FOREIGN KEY (owner_id) REFERENCES investment_users(id)
);

CREATE TABLE investment_types (
    id serial NOT NULL PRIMARY KEY,
    type_name VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE investments (
    id UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
    investment_name VARCHAR(255) NOT NULL,
    code VARCHAR(30),
    initial_value DECIMAL(15, 2) NOT NULL,
    current_value DECIMAL(15, 2) NOT NULL,
    investment_datetime TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    closed BOOLEAN NOT NULL DEFAULT FALSE,
    deleted BOOLEAN NOT NULL DEFAULT FALSE,
    group_id UUID NOT NULL,
    creator_id UUID NOT NULL,
    investment_type_id serial,
    FOREIGN KEY (group_id) REFERENCES investment_groups(id),
    FOREIGN KEY (creator_id) REFERENCES investment_users(id),
    FOREIGN KEY (investment_type_id) REFERENCES investment_types(id)
);
