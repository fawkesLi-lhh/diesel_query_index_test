mod models;
mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use crate::schema::posts::dsl::posts;
use models::NewPost;
use std::time::Instant;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = String::from("test.db");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_post(conn: &mut SqliteConnection, title: &str, body: &str) -> usize {
    use crate::schema::posts;

    let mut new_data = Vec::new();
    for i in 1..1111111{
        new_data.push({
            NewPost {
                id1: i%111,
                id2: i,
                title,
                body,
                published: i%3,
            }
        })
    }

    diesel::insert_into(posts::table)
        .values(new_data)
        .execute(conn)
        .expect("Error saving new post")
}

fn main() {
    use self::schema::posts::dsl::*;
    let connection = &mut establish_connection();

    //let _ = create_post(connection, "aa", "bb");
    let start = Instant::now();
    for i in 0..111 {
        let results = posts
            .filter(published.eq(2))
            .filter(id1.eq(i))
            .load::<models::Post>(connection)
            .expect("Error loading posts");
        //println!("Displaying {} posts", results.len());
    }
    println!("time cost: {:?} ms", start.elapsed().as_millis());// ms


    // for post in results {
    //     println!("{:?}", post);
    // }
}
