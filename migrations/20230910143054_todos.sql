create table if not exists todos (
    id integer primary key autoincrement not null,
    body text not null,
    completed boolean not null default false,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);