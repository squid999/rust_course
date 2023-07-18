-- Your SQL goes here
create table if not exists stu_clubs
(
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  sid           int not null,
  clubid        int not null,
  comment       varchar(255),
  created_at  datetime default current_timestamp not null,
  updated_at datetime default current_timestamp not null
);