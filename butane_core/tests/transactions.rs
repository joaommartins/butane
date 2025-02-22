use butane_core::db::{BackendConnection, Connection};

use butane_test_helper::*;

fn commit_empty_transaction(mut conn: Connection) {
    assert!(!conn.is_closed());

    let tr = conn.transaction().unwrap();

    assert!(tr.commit().is_ok());
    // it is impossible to reuse the transaction after this.
    // i.e. already_consumed is unreachable.
}
testall_no_migrate!(commit_empty_transaction);

fn rollback_empty_transaction(mut conn: Connection) {
    let tr = conn.transaction().unwrap();

    assert!(tr.rollback().is_ok());
    // it is impossible to reuse the transaction after this.
    // i.e. already_consumed is unreachable.
}
testall_no_migrate!(rollback_empty_transaction);

fn debug_transaction_before_consuming(mut conn: Connection) {
    let backend_name = conn.backend_name().clone();

    let tr = conn.transaction().unwrap();

    if backend_name == "pg" {
        assert!(format!("{:?}", tr).contains("{ trans: true }"));
    } else {
        assert!(format!("{:?}", tr).contains("path: Some(\"\")"));
    }

    assert!(tr.commit().is_ok());
}
testall_no_migrate!(debug_transaction_before_consuming);
