-- Your SQL goes here
CREATE TABLE thumbs (
    picture_id INTEGER PRIMARY KEY UNIQUE NOT NULL,
    picture_hash VARCHAR(40) NOT NULL,
    FOREIGN KEY(picture_id) REFERENCES pictures(id) ON DELETE CASCADE
)