create table users
(
    id            integer not null
        constraint id_pk
            primary key autoincrement,
    email         text    not null,
    display_name  text    not null,
    safe_username text    not null,
    password_hash text    not null
);

create unique index email_ui
    on users (email);

create unique index display_name_ui
    on users (display_name);

create unique index safe_username_ui
    on users (safe_username);