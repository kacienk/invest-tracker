CREATE TABLE investment_users (
    id serial NOT NULL PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE CHECK (email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}$'),
    password VARCHAR(255) NOT NULL,
    salt VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE investment_groups (
    id serial NOT NULL PRIMARY KEY,
    group_name VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    deleted BOOLEAN NOT NULL DEFAULT FALSE,
    owner_id INT NOT NULL,
    FOREIGN KEY (owner_id) REFERENCES investment_users(id)
);

CREATE TABLE investment_types (
    id serial NOT NULL PRIMARY KEY,
    type_name VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE investments (
    id serial NOT NULL PRIMARY KEY,
    investment_name VARCHAR(255) NOT NULL,
    code VARCHAR(30),
    initial_value DECIMAL(15, 2) NOT NULL,
    current_value DECIMAL(15, 2) NOT NULL,
    investment_datetime TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    closed BOOLEAN NOT NULL DEFAULT FALSE,
    deleted BOOLEAN NOT NULL DEFAULT FALSE,
    group_id INT NOT NULL,
    creator_id INT NOT NULL,
    investment_type_id INT,
    FOREIGN KEY (group_id) REFERENCES investment_groups(id),
    FOREIGN KEY (creator_id) REFERENCES investment_users(id),
    FOREIGN KEY (investment_type_id) REFERENCES investment_types(id)
);
