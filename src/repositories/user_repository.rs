use diesel::prelude::*;
use diesel::PgConnection;

use crate::models::user::{NewUser, User};
use crate::schema::users;

pub fn find_all_paginated(
    conn: &mut PgConnection,
    page: i64,
    per_page: i64,
) -> QueryResult<(Vec<User>, i64)> {
    let offset = (page - 1) * per_page;

    let total = users::table.count().get_result::<i64>(conn)?;

    let items = users::table
        .order(users::created_at.desc())
        .offset(offset)
        .limit(per_page)
        .load::<User>(conn)?;

    Ok((items, total))
}

pub fn insert(conn: &mut PgConnection, new_user: &NewUser) -> QueryResult<User> {
    diesel::insert_into(users::table)
        .values(new_user)
        .get_result(conn)
}

pub fn find_by_email(conn: &mut PgConnection, email: &str) -> QueryResult<User> {
    users::table
        .filter(users::email.eq(email))
        .first::<User>(conn)
}

pub fn find_by_id(conn: &mut PgConnection, user_id: uuid::Uuid) -> QueryResult<User> {
    users::table.find(user_id).first::<User>(conn)
}
