-- Your SQL goes here
CREATE TABLE User 
(
    rsa_key VARCHAR(200) PRIMARY KEY NOT NULL,
    username VARCHAR(100) NOT NULL,
    password VARCHAR(200) NOT NULL,
    email VARCHAR(200) NOT NULL,
    createdAt TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    banner VARCHAR(400) NOT NULL
);

CREATE TABLE Messages
(
    author_key VARCHAR(200),
    recipent_key VARCHAR(200),
    id INT PRIMARY KEY AUTO_INCREMENT NOT NULL,
    createdAt TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    message_content VARCHAR(10000) NOT NULL,
    FOREIGN KEY (author_key) REFERENCES User(rsa_key),
    FOREIGN KEY (recipent_key) REFERENCES User(rsa_key)
);