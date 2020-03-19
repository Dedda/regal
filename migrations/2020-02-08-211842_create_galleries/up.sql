-- Your SQL goes here
CREATE TABLE gallerys (
    id  INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(200) NOT NULL,
    directory VARCHAR(255) UNIQUE,
    parent INTEGER,
    FOREIGN KEY(parent) REFERENCES gallerys(id),
    UNIQUE(name, parent)
);

PRAGMA foreign_keys=off;

ALTER TABLE images RENAME TO _images_old;

CREATE TABLE images (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(200) NOT NULL,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    gallery_id INTEGER NOT NULL,
    format VARCHAR(10) NOT NULL,
    path VARCHAR(255) NOT NULL,
    sha1 VARCHAR(40) NOT NULL,
    filesize INTEGER NOT NULL,
    external_id VARCHAR(40) UNIQUE NOT NULL,
    FOREIGN KEY(gallery_id) REFERENCES gallerys(id)
);

INSERT INTO images SELECT * FROM _images_old;

DROP TABLE _images_old;

PRAGMA foreign_keys=on;