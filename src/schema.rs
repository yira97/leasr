table! {
    post_change (id) {
        id -> Int8,
        post_id -> Int8,
        created_at -> Timestamp,
        change_name -> Varchar,
        old -> Text,
        new -> Text,
    }
}

table! {
    post_tag (name) {
        name -> Varchar,
        created_at -> Timestamp,
        disable_at -> Timestamp,
    }
}

table! {
    posts (id) {
        id -> Int8,
        title -> Varchar,
        describe -> Text,
        content -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        disable_at -> Timestamp,
        creator -> Int8,
        tag -> Nullable<Varchar>,
    }
}

table! {
    users (id) {
        id -> Int8,
        username -> Varchar,
        display_name -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        disable_at -> Timestamp,
        score -> Int4,
        weight -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    post_change,
    post_tag,
    posts,
    users,
);
