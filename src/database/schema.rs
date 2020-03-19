table! {
    gallerys (id) {
        id -> Integer,
        name -> Text,
        directory -> Nullable<Text>,
        parent -> Nullable<Integer>,
    }
}

table! {
    picture_tags (tag_id, picture_id) {
        tag_id -> Integer,
        picture_id -> Integer,
    }
}

table! {
    pictures (id) {
        id -> Integer,
        name -> Text,
        width -> Integer,
        height -> Integer,
        gallery_id -> Integer,
        format -> Text,
        path -> Text,
        sha1 -> Text,
        filesize -> Integer,
        external_id -> Text,
    }
}

table! {
    tags (id) {
        id -> Integer,
        tag_type -> Integer,
        name -> Text,
    }
}

table! {
    thumbs (picture_id) {
        picture_id -> Integer,
        picture_hash -> Text,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Text,
        email -> Text,
        password -> Text,
        verification -> Nullable<Text>,
    }
}

joinable!(pictures -> gallerys (gallery_id));
joinable!(thumbs -> pictures (picture_id));

allow_tables_to_appear_in_same_query!(
    gallerys,
    picture_tags,
    pictures,
    tags,
    thumbs,
    users,
);
