use super::schema::posts;
use diesel::prelude::*;

#[derive(Queryable,Debug)]
pub struct Post {
    pub id1: i32,
    pub id2: i32,
    pub title: String,
    pub body: String,
    pub published: i32,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub id1: i32,
    pub id2: i32,
    pub title: &'a str,
    pub body: &'a str,
    pub published: i32,
}