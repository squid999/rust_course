// @generated automatically by Diesel CLI.

diesel::table! {
    clubs (id) {
        id -> Integer,
        name -> Nullable<Text>,
        description -> Nullable<Text>,
        create_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    stu_clubs (id) {
        id -> Integer,
        sid -> Integer,
        clubid -> Integer,
        comment -> Nullable<Text>,
        create_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    students (id) {
        id -> Integer,
        name -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(clubs, stu_clubs, students,);
