table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        first -> Varchar,
        last -> Varchar,
        created -> Timestamp,
        deleted -> Nullable<Timestamp>,
        password -> Bytea,
        salt -> Bytea,
        num_iterations -> Int4,
        owner -> Bool,
    }
}
