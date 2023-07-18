-- Your SQL goes here
create table if not exists classes
(
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  clsid        varchar(50) unique not null,
  name         varchar(50),
  created_at  datetime default current_timestamp not null,
  updated_at datetime default current_timestamp not null
)