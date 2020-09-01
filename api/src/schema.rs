table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        first -> Varchar,
        last -> Varchar,
        created -> Timestamp,
        deleted -> Nullable<Timestamp>,
        password -> Varchar,
        owner -> Bool,
    }
}
