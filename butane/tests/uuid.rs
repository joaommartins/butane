use butane::model;
use butane::prelude::*;
use butane::{db::Connection, ObjectState};
use uuid_for_test::Uuid;

use butane_test_helper::*;

#[model]
#[derive(PartialEq, Eq, Debug, Clone)]
struct FooUU {
    id: Uuid,
    bar: u32,
}
impl FooUU {
    fn new(id: Uuid) -> Self {
        FooUU {
            id,
            bar: 0,
            state: ObjectState::default(),
        }
    }
}

fn basic_uuid(conn: Connection) {
    //create
    let id = Uuid::new_v4();
    let mut foo = FooUU::new(id);
    foo.bar = 42;
    foo.save(&conn).unwrap();

    // read
    let mut foo2 = FooUU::get(&conn, id).unwrap();
    assert_eq!(foo, foo2);

    // update
    foo2.bar = 43;
    foo2.save(&conn).unwrap();
    let foo3 = FooUU::get(&conn, id).unwrap();
    assert_eq!(foo2, foo3);
}
testall!(basic_uuid);
