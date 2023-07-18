-- Your SQL goes here
create table if not exists stu_class
(
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  sid          varchar(60) not null UNIQUE,
  clsid        varchar(60) not null,
  comment       varchar(255),
  created_at  datetime default current_timestamp not null,
  updated_at datetime default current_timestamp not null
);