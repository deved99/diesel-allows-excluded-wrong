use chrono::{DateTime, Utc};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::upsert::excluded;
use dotenvy::dotenv;
use uuid::Uuid;

mod schema;

#[derive(Debug, Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct Post {
    title: String,
    created: DateTime<Utc>,
    modified: DateTime<Utc>,
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn main() {
    use schema::posts::dsl::*;

    let mut db = establish_connection();
    let mut old_posts: Vec<Post> = posts.limit(5).load(&mut db).unwrap();

    // Populate table
    if old_posts.is_empty() {
        old_posts = (0..5)
            .map(|_| Post {
                title: Uuid::new_v4().to_string(),
                created: Utc::now(),
                modified: Utc::now(),
            })
            .collect();
        diesel::insert_into(schema::posts::table)
            .values(&old_posts)
            .execute(&mut db)
            .expect("Failed inserting old_posts");
    }

    let new_posts: Vec<Post> = old_posts
        .into_iter()
        .map(|mut p| {
            p.modified = Utc::now();
            p
        })
        .collect()
    ;

    diesel::insert_into(posts)
        .values(new_posts)
        .on_conflict(title)
        .filter_target(modified.lt(excluded(modified)))
        .do_update()
        .set(modified.eq(excluded(modified)))
        .execute(&mut db)
        .expect("Failure inserting new_posts");
}
