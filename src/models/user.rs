use diesel::prelude::*;
use chrono::{DateTime, Utc};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User{
    id: String,
    name: String,
    email: String,
    password: String,
    created_at: DateTime<Utc>,
}