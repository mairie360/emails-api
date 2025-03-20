-- Your SQL goes here
-- CREATE TABLE users (
--     id SERIAL PRIMARY KEY,
--     email VARCHAR(320) NOT NULL,
--     password VARCHAR(60) NOT NULL,
--     created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
-- );
-- CREATE TABLE user_mail_boxes (
--     user_id INT NOT NULL,
--     token JSONB NOT NULL,
--     type enum('gmail', 'outlook', 'orange', 'custom') NOT NULL,
--     FOREIGN KEY (user_id) REFERENCES users(id),
--     PRIMARY KEY (user_id)
-- );
CREATE TABLE
    emails (
        id SERIAL PRIMARY KEY,
        sender VARCHAR(320) NOT NULL,
        recipient VARCHAR(320) NOT NULL,
        subject TEXT,
        body TEXT,
        is_read BOOLEAN DEFAULT FALSE,
        received_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );

-- CREATE TABLE email_mail_box (
--     email_id INT NOT NULL,
--     mail_box_id INT NOT NULL,
--     FOREIGN KEY (email_id) REFERENCES emails(id),
--     FOREIGN KEY (mail_box_id) REFERENCES user_mail_boxes(id),
--     PRIMARY KEY (email_id, mail_box_id)
-- );