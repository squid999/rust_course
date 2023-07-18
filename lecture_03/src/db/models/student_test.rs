use crate::db::{establish_connection, models::Student};

#[test]
fn create_student_with_name() {
    let conn = establish_connection().get().unwrap();
    let name = Some("Tom");

    let stu = Student::create(name, &conn).unwrap();
    // println!("!!!! {:?}", stu);
    assert_eq!(stu.name.unwrap().as_str(), name.unwrap());
}

#[test]
fn create_student_with_existing_name() {
    let conn = establish_connection().get().unwrap();
    let name = Some("Tom");
    let user = Student::create(name, &conn).unwrap();
    let existing_stu = Student::create(name, &conn).unwrap();

    assert_eq!(user.id, existing_stu.id);
}

// #[test]
// fn create_user_with_phone_only() {
//     let conn = establish_connection().get().unwrap();
//     let email = None;
//     let phone = Some("123456789");

//     let user = User::create(email, phone, &conn).unwrap();

//     assert!(user.email.is_none());
//     assert_eq!(user.phone.unwrap().as_str(), phone.unwrap());
// }

// #[test]
// fn create_user_with_email_only() {
//     let conn = establish_connection().get().unwrap();
//     let email = Some("test@email.com");
//     let phone = None;

//     let user = User::create(email, phone, &conn).unwrap();

//     assert_eq!(user.email.unwrap().as_str(), email.unwrap());
//     assert!(user.phone.is_none());
// }

// #[test]
// fn create_user_with_existing_email() {
//     let conn = establish_connection().get().unwrap();
//     let email = Some("test@email.com");
//     let phone = None;

//     let user = User::create(email, phone, &conn).unwrap();
//     let existing_user = User::create(email, phone, &conn).unwrap();

//     assert_eq!(user.id, existing_user.id);
// }

// #[test]
// fn create_user_with_existing_phone() {
//     let conn = establish_connection().get().unwrap();
//     let email = None;
//     let phone = Some("123456789");

//     let user = User::create(email, phone, &conn).unwrap();
//     let existing_user = User::create(email, phone, &conn).unwrap();

//     assert_eq!(user.id, existing_user.id);
// }

// #[test]
// fn list_students() {
//     let conn = establish_connection().get().unwrap();
//     let name1 = Some("Tom");
//     let name2 = Some("Jerry");

//     let user = Student::create(name1, &conn).unwrap();
//     let user2 = Student::create(name2, &conn).unwrap();

//     let existing_users = Student::list(&conn);

//     println!("{:?}", existing_users);
//     assert_eq!(2, existing_users.len());
//     assert_eq!(user.id, existing_users[0].id);
// }

// #[test]
// fn get_user_by_phone() {
//     let conn = establish_connection().get().unwrap();
//     let email = None;
//     let phone = Some("123456789");

//     let user = User::create(email, phone, &conn).unwrap();
//     let existing_user = User::by_phone(&phone.unwrap(), &conn).unwrap();

//     assert_eq!(user.id, existing_user.id);
// }

// #[test]
// fn get_user_by_email() {
//     let conn = establish_connection().get().unwrap();
//     let email = Some("test@email.com");
//     let phone = None;

//     let user = User::create(email, phone, &conn).unwrap();
//     let existing_user = User::by_email(&email.unwrap(), &conn).unwrap();

//     assert_eq!(user.id, existing_user.id);
// }

// #[test]
// fn get_user_by_id() {
//     let conn = establish_connection().get().unwrap();
//     let email = Some("test@email.com");
//     let phone = Some("123456789");

//     let user = User::create(email, phone, &conn).unwrap();
//     let existing_user = User::by_id(&user.id, &conn).unwrap();

//     assert_eq!(user.id, existing_user.id);
// }
