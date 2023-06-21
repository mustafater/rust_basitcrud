

CREATE TABLE
    IF NOT EXISTS Products(
        id varchar not null,
        name VARCHAR(255) NOT NULL UNIQUE,
        description  VARCHAR NOT NULL,
        quantity VARCHAR(100),
        price VARCHAR NOT NULL

    );