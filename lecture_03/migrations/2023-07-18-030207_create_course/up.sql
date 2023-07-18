-- Your SQL goes here
create table if not exists courses
(
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  cid        varchar(50) unique,
  name         varchar(50),
  description  varchar(255),
  created_at  datetime default current_timestamp not null,
  updated_at datetime default current_timestamp not null
)