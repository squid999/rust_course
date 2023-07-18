use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::schema::students;
use super::schema::students::dsl::students as stu_dsl;

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "students"]
pub struct StudentForm<'a> {
    name: Option<&'a str>,
}
#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "students"]
pub struct Student {
    pub id: i32,
    pub name: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl Student {
    pub fn list(conn: &SqliteConnection) -> Vec<Self> {
        stu_dsl
            .load::<Student>(conn)
            .expect("Error loading students")
    }

    pub fn by_id(id: i32, conn: &SqliteConnection) -> Option<Self> {
        stu_dsl.find(id).get_result::<Student>(conn).ok()
    }

    pub fn by_name(name_str: &str, conn: &SqliteConnection) -> Option<Self> {
        use super::schema::students::dsl::name;
        println!("???????");
        stu_dsl
            .filter(name.eq(name_str))
            .first::<Student>(conn)
            .ok()
    }

    pub fn create(name: Option<&str>, conn: &SqliteConnection) -> Option<Self> {
        if name.is_none() {
            return None;
        }

        if name.is_some() {
            if let Some(stu) = Self::by_name(&name.unwrap(), conn) {
                return Some(stu);
            }
        }

        let new_stu = Self::new_stu_struct(name);
        println!("{:?}", new_stu);

        diesel::insert_into(stu_dsl)
            .values(&new_stu)
            .execute(conn)
            .expect("Error saving new students");
        println!("here {}", name.unwrap());
        Self::by_name(&name.unwrap(), conn)
    }

    fn new_stu_struct(name: Option<&str>) -> StudentForm {
        StudentForm {
            name: name.map(Into::into),
        }
    }
}

#[cfg(test)]
mod student_test;
