// @generated automatically by Diesel CLI.

diesel::table! {
    classes (id) {
        id -> Nullable<Integer>,
        clsid -> Text,
        name -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    clubs (id) {
        id -> Nullable<Integer>,
        name -> Nullable<Text>,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    courses (id) {
        id -> Nullable<Integer>,
        cid -> Nullable<Text>,
        name -> Nullable<Text>,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    stu_class (id) {
        id -> Nullable<Integer>,
        sid -> Text,
        clsid -> Text,
        comment -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    stu_clubs (id) {
        id -> Nullable<Integer>,
        sid -> Integer,
        clubid -> Integer,
        comment -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    students (id) {
        id -> Nullable<Integer>,
        sid -> Text,
        name -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    classes, clubs, courses, stu_class, stu_clubs, students,
);
