create table if not exists version
(
    type    text not null,
    version text not null default '0.0.1'
);
create table if not exists user
(
    id              integer primary key autoincrement,
    username        text     not null unique,
    password        text     not null,
    is_admin        boolean  not null default false,
    last_login_time datetime
);

create table if not exists user_session
(
    id          integer primary key autoincrement,
    user_id     int      not null,
    token       text     not null unique,
    create_time datetime not null,
    ip          text not null
);

create table if not exists category
(
    id      integer primary key autoincrement,
    user_id integer not null,
    title   text    not null,
    unique (user_id, title)
);

create table if not exists  feed
(
    id                  integer primary key autoincrement,
    user_id             int      not null,
    category_id         int      not null,
    title               text     not null,
    feed_url            text     not null,
    site_url            text     not null,
    icon                blob,
    checked_time        datetime not null,
    etag_header         text default '',
    parsing_error_msg   text default '',
    parsing_error_count int  default 0,
    unique (user_id, feed_url)
);

create table if not exists  entry
(
    id           integer primary key autoincrement,
    user_id      integer  not null,
    feed_id      integer  not null,
    hash         text     not null,
    publish_time datetime not null,
    title        text     not null,
    url          text     not null,
    author       text,
    content      text,
    state        text check (state in ('unread', 'read', 'remove')),
    unique (feed_id, hash)
);