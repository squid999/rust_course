-- Your SQL goes here
create table if not exists clubs
(
  id          INTEGER PRIMARY KEY AUTOINCREMENT,
  name         varchar(50) unique,
  description  varchar(255),
  created_at  datetime default current_timestamp not null,
  updated_at datetime default current_timestamp not null
)