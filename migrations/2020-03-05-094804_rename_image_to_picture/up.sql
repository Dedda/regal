-- Your SQL goes here
ALTER TABLE images RENAME TO pictures;

PRAGMA foreign_keys=off;

ALTER TABLE image_tags RENAME TO _image_tags_old;

CREATE TABLE picture_tags (
    tag_id INTEGER NOT NULL,
    picture_id INTEGER NOT NULL,
    CONSTRAINT picture_tag_pk PRIMARY KEY(tag_id, picture_id),
    FOREIGN KEY(tag_id) REFERENCES tag(id) ON DELETE CASCADE,
    FOREIGN KEY(picture_id) REFERENCES picture(id) ON DELETE CASCADE
);

INSERT INTO picture_tags (tag_id, picture_id)
  SELECT tag_id, image_id
  FROM _image_tags_old;

PRAGMA foreign_keys=on;

DROP TABLE _image_tags_old;