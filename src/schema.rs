table! {
    users (id) {
        id -> Uuid,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        pin_code -> Int2,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
