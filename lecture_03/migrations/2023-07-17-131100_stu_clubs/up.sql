-- Your SQL goes here
create table if not exists stu_clubs
(
  id           int auto_increment primary key,
  sid           int not null,
  clubid        int not null,
  comment       varchar(255),
  create_at  datetime default current_timestamp not null,
  updated_at datetime default current_timestamp not null
);