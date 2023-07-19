use crate::db::{establish_connection, models::Student};

#[test]
fn create_student_with_name() {
    let conn = establish_connection().get().unwrap();
    let sid = "001";
    let name = Some("Tom");

    let stu = Student::create(sid, name, &conn).unwrap();
    // println!("!!!! {:?}", stu);
    assert_eq!(stu.name.unwrap().as_str(), name.unwrap());
}

#[test]
fn create_student_with_existing_name() {
    let conn = establish_connection().get().unwrap();
    let sid = "001";
    let name = Some("Tom");
    let user = Student::create(sid, name, &conn).unwrap();
    let existing_stu = Student::create(sid, name, &conn).unwrap();

    assert_eq!(user.id, existing_stu.id);
}

#[test]
fn list_students() {
    let conn = establish_connection().get().unwrap();
    let name1 = Some("Tom");
    let sid1 = "001";

    let name2 = Some("Jerry");
    let sid2 = "002";

    let user = Student::create(sid1, name1, &conn).unwrap();
    let user2 = Student::create(sid2, name2, &conn).unwrap();

    let existing_users = Student::list(&conn);

    println!("{:?}", existing_users);
    assert_eq!(2, existing_users.len());
    assert_eq!(user.id, existing_users[0].id);
}

#[test]
fn get_stu_by_sid() {
    let conn = establish_connection().get().unwrap();
    let sid = "001";
    let name = Some("Tom");

    let stu = Student::create(sid, name, &conn).unwrap();
    let existing_stu = Student::by_sid(sid, &conn).unwrap();

    assert_eq!(stu.id, existing_stu.id);
}

#[test]
fn get_stu_by_email() {
    let conn = establish_connection().get().unwrap();
    let sid = "001";
    let name = Some("Tom");

    let stu = Student::create(sid, name, &conn).unwrap();
    let existing_stu = Student::by_name(name.unwrap(), &conn).unwrap();

    assert_eq!(stu.id, existing_stu.id);
}

#[test]
fn get_stu_by_id() {
    let conn = establish_connection().get().unwrap();
    let sid = "001";
    let name = Some("Tom");

    let stu = Student::create(sid, name, &conn).unwrap();
    let existing_stu = Student::by_id(stu.id.unwrap(), &conn).unwrap();

    assert_eq!(stu.id, existing_stu.id);
}
#[test]
fn del_stu_by_sid() {
    let conn = establish_connection().get().unwrap();
    let sid = "001";
    let name = Some("Tom");

    let stu = Student::create(sid, name, &conn).unwrap();

    Student::remove(sid, &conn).unwrap();
    let existing_stu = Student::by_sid(sid, &conn);
    match existing_stu {
        Some(_) => assert!(false),
        None => assert!(true),
    }
}

#[test]
fn update_name_by_sid() {
    let conn = establish_connection().get().unwrap();
    let sid = "001";
    let name = Some("Tom");

    let stu = Student::create(sid, name, &conn).unwrap();

    Student::update(sid, Some("Jim"), &conn);

    let existing_stu = Student::by_sid(sid, &conn).unwrap();
    assert_eq!(existing_stu.name.as_deref(), Some("Jim"));
}
