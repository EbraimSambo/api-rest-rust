use diesel::prelude::*;
use diesel::PgConnection;

use crate::models::user::User;
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
