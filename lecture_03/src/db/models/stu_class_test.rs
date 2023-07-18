use crate::db::{establish_connection, models::StuClass};

#[test]
fn create_stuclass_with_sid_and_clsid() {
    let conn = establish_connection().get().unwrap();
    let sid = "001";
    let clsid = "3-001";
    let comment = Some("test");

    let stu = StuClass::create(sid, clsid, comment, &conn).unwrap();
    // println!("!!!! {:?}", stu);
    assert_eq!(stu.sid, sid);
}

#[test]
fn list_entries() {
    let conn = establish_connection().get().unwrap();

    let stu1 = StuClass::create("001", "3-001", Some("entry-1"), &conn).unwrap();
    let stu2 = StuClass::create("002", "3-001", Some("entry-2"), &conn).unwrap();
    let stu3 = StuClass::create("003", "3-002", Some("entry-3"), &conn).unwrap();

    let existing_entries = StuClass::list(&conn);
    println!("{:?}", existing_entries);
    assert_eq!(3, existing_entries.len());
    assert_eq!(stu1.id, existing_entries[0].id);
}

#[test]
fn remove_class() {
    let conn = establish_connection().get().unwrap();

    let stu1 = StuClass::create("001", "3-001", Some("entry-1"), &conn).unwrap();
    let stu2 = StuClass::create("002", "3-001", Some("entry-2"), &conn).unwrap();
    let stu3 = StuClass::create("003", "3-002", Some("entry-3"), &conn).unwrap();

    StuClass::remove_cls("3-001", &conn).unwrap();
    let existing_entries = StuClass::list(&conn);
    println!("{:?}", existing_entries);
    assert_eq!(1, existing_entries.len());
    assert_eq!(stu3.id, existing_entries[0].id);
}
