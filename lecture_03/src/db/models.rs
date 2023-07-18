use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::schema::students;
use super::schema::students::dsl::students as stu_dsl;

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "students"]
pub struct StudentForm {
    sid: String,
    name: Option<String>,
}
// pub struct StudentForm<'a> {
//     sid: &'a str,
//     name: Option<&'a str>,
// }
#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "students"]
pub struct Student {
    pub id: Option<i32>,
    pub sid: String,
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

    pub fn by_sid(sid_str: &str, conn: &SqliteConnection) -> Option<Self> {
        use super::schema::students::dsl::sid;

        stu_dsl.filter(sid.eq(sid_str)).first::<Student>(conn).ok()
    }

    pub fn by_name(name_str: &str, conn: &SqliteConnection) -> Option<Self> {
        use super::schema::students::dsl::name;

        stu_dsl
            .filter(name.eq(name_str))
            .first::<Student>(conn)
            .ok()
    }

    pub fn create(sid: &str, name: Option<&str>, conn: &SqliteConnection) -> Option<Self> {
        if name.is_none() {
            return None;
        }

        if name.is_some() {
            if let Some(stu) = Self::by_name(&name.unwrap(), conn) {
                return Some(stu);
            }
        }

        // let new_stu = Self::new_stu_struct(name);
        let new_stu = Self::new_stu_struct(sid, name);
        println!("{:?}", new_stu);

        diesel::insert_into(stu_dsl)
            .values(&new_stu)
            .execute(conn)
            .expect("Error saving new students");
        println!("here {}", name.unwrap());
        Self::by_sid(&sid, conn)
    }

    pub fn remove(sid_str: &str, conn: &SqliteConnection) -> QueryResult<usize> {
        use super::schema::students::dsl::sid;
        diesel::delete(stu_dsl.filter(sid.eq(sid_str))).execute(conn)
    }

    fn new_stu_struct(sid: &str, name: Option<&str>) -> StudentForm {
        StudentForm {
            sid: sid.to_string(),
            name: name.map(Into::into),
        }
    }
}

use super::schema::clubs;
use super::schema::clubs::dsl::clubs as club_dsl;

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "clubs"]
pub struct ClubForm<'a> {
    name: Option<&'a str>,
}

use super::schema::classes;
use super::schema::classes::dsl::classes as cls_dsl;

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "classes"]
pub struct ClassForm {
    clsid: String,
    name: Option<String>,
}
#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "classes"]
pub struct Class {
    pub id: Option<i32>,
    pub clsid: String,
    pub name: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl Class {
    pub fn list(conn: &SqliteConnection) -> Vec<Self> {
        cls_dsl.load::<Class>(conn).expect("Error loading students")
    }

    pub fn by_id(id: i32, conn: &SqliteConnection) -> Option<Self> {
        cls_dsl.find(id).get_result::<Class>(conn).ok()
    }

    pub fn by_clsid(clsid_str: &str, conn: &SqliteConnection) -> Option<Self> {
        use super::schema::classes::dsl::clsid;

        cls_dsl
            .filter(clsid.eq(clsid_str))
            .first::<Class>(conn)
            .ok()
    }

    pub fn by_name(name_str: &str, conn: &SqliteConnection) -> Option<Self> {
        use super::schema::classes::dsl::name;

        cls_dsl.filter(name.eq(name_str)).first::<Class>(conn).ok()
    }

    pub fn create(clsid: &str, name: Option<&str>, conn: &SqliteConnection) -> Option<Self> {
        if name.is_none() {
            return None;
        }

        if name.is_some() {
            if let Some(stu) = Self::by_name(&name.unwrap(), conn) {
                return Some(stu);
            }
        }

        // let new_stu = Self::new_stu_struct(name);
        let new_cls = Self::new_cls_struct(clsid, name);
        // println!("{:?}", new_stu);

        diesel::insert_into(cls_dsl)
            .values(&new_cls)
            .execute(conn)
            .expect("Error saving new students");
        println!("here {}", name.unwrap());
        Self::by_clsid(&clsid, conn)
    }

    pub fn remove(clsid_str: &str, conn: &SqliteConnection) -> QueryResult<usize> {
        use super::schema::classes::dsl::clsid;
        diesel::delete(cls_dsl.filter(clsid.eq(clsid_str))).execute(conn)
    }

    fn new_cls_struct(clsid: &str, name: Option<&str>) -> ClassForm {
        ClassForm {
            clsid: clsid.to_string(),
            name: name.map(Into::into),
        }
    }
}

use super::schema::stu_class;
use super::schema::stu_class::dsl::stu_class as sc_dsl;

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "stu_class"]
pub struct StuClassForm {
    sid: String,
    clsid: String,
    comment: Option<String>,
}
#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "stu_class"]
pub struct StuClass {
    pub id: Option<i32>,
    pub sid: String,
    pub clsid: String,
    pub comment: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl StuClass {
    pub fn list(conn: &SqliteConnection) -> Vec<Self> {
        sc_dsl
            .load::<StuClass>(conn)
            .expect("Error loading students")
    }

    pub fn by_id(id: i32, conn: &SqliteConnection) -> Option<Self> {
        sc_dsl.find(id).get_result::<StuClass>(conn).ok()
    }

    pub fn by_clsid(clsid_str: &str, conn: &SqliteConnection) -> Option<Vec<Self>> {
        use super::schema::stu_class::dsl::clsid;

        sc_dsl
            .filter(clsid.eq(clsid_str))
            .limit(100)
            .load(conn)
            .ok()
    }

    pub fn by_sid(sid_str: &str, conn: &SqliteConnection) -> Option<Self> {
        use super::schema::stu_class::dsl::sid;

        sc_dsl.filter(sid.eq(sid_str)).first(conn).ok()
    }

    pub fn create(
        sid: &str,
        clsid: &str,
        cmt: Option<&str>,
        conn: &SqliteConnection,
    ) -> Option<Self> {
        if clsid.is_empty() {
            return None;
        }

        if sid.is_empty() {
            return None;
        } else {
            if let Some(stucls) = Self::by_sid(sid, conn) {
                return Some(stucls);
            }
        }

        // let new_stu = Self::new_stu_struct(name);
        let new_cls = Self::new_stucls_struct(sid, clsid, cmt);
        // println!("{:?}", new_stu);

        diesel::insert_into(sc_dsl)
            .values(&new_cls)
            .execute(conn)
            .expect("Error saving new students");
        Self::by_sid(&sid, conn)
    }

    pub fn remove(sid_str: &str, conn: &SqliteConnection) -> QueryResult<usize> {
        use super::schema::stu_class::dsl::sid;
        diesel::delete(sc_dsl.filter(sid.eq(sid_str))).execute(conn)
    }

    pub fn remove_cls(clsid_str: &str, conn: &SqliteConnection) -> QueryResult<usize> {
        use super::schema::stu_class::dsl::clsid;
        diesel::delete(sc_dsl.filter(clsid.eq(clsid_str))).execute(conn)
    }

    fn new_stucls_struct(sid: &str, clsid: &str, cmt: Option<&str>) -> StuClassForm {
        StuClassForm {
            sid: sid.to_string(),
            clsid: clsid.to_string(),
            comment: cmt.map(Into::into),
        }
    }
}

#[cfg(test)]
mod student_test;

#[cfg(test)]
mod stu_class_test;
