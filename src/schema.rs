diesel::table! {
    users (id) {
        id -> VarChar,
        name -> VarChar,
        email -> VarChar,
        password -> VarChar,
        created_at -> Timestamptz,
    }
}
