-- Your SQL goes here
create table if not exists clubs
(
  id           int auto_increment primary key not null,
  name         varchar(50) unique,
  description  varchar(255),
  create_at  datetime default current_timestamp not null,
  updated_at datetime default current_timestamp not null
)