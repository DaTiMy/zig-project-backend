CREATE TABLE zigs (
    id VARCHAR(255) NOT NULL ,
    user_name VARCHAR(255) NOT NULL,
    button_counter INT DEFAULT 0,
    ash_counter INT DEFAULT 0,
    PRIMARY KEY (id)
);