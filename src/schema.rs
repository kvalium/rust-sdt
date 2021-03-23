table! {
    users (id) {
        id -> Uuid,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
