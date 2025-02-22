#![allow(unused_imports)]

use butane::db::Connection;
use butane::{find, DataObject, ForeignKey};

use butane_test_helper::*;

mod common;

#[cfg(feature = "fake")]
fn fake_blog_post(conn: Connection) {
    use fake::{Fake, Faker};

    use common::blog::{Blog, Post, Tag};

    let mut fake_blog: Blog = Faker.fake();
    fake_blog.save(&conn).unwrap();

    let mut post: Post = Faker.fake();
    post.blog = ForeignKey::<Blog>::from(fake_blog);

    let mut tag_1: Tag = Faker.fake();
    tag_1.save(&conn).unwrap();
    let mut tag_2: Tag = Faker.fake();
    tag_2.save(&conn).unwrap();
    let mut tag_3: Tag = Faker.fake();
    tag_3.save(&conn).unwrap();

    post.tags.add(&tag_1).unwrap();
    post.tags.add(&tag_2).unwrap();
    post.tags.add(&tag_3).unwrap();
    post.save(&conn).unwrap();

    let post_from_db = find!(Post, id == { post.id }, &conn).unwrap();
    assert_eq!(post_from_db.title, post.title);
    assert_eq!(post_from_db.tags.load(&conn).unwrap().count(), 3);
}
#[cfg(feature = "fake")]
testall!(fake_blog_post);
