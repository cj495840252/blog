-- Add migration script here
create table IF NOT EXISTS users (
    id serial primary key,
    username varchar(64) not null, -- 如果用的是github授权，这个字段就取邮箱@前面部分
    email varchar(64) not null unique, -- 用户邮箱
    password varchar(255) -- password hash
);
create table IF NOT EXISTS articles (
    id serial primary key,
    title varchar(100) not null,
    description varchar(255) not null, -- store the blog description
    classification varchar(20) not null, -- store the blog classification，like frontend, backend, etc.
    catagory varchar(20) not null, -- store the blog catagory，like tokio, axum, sqlx, etc.
    content text not null, -- store the blog content
    cover_image varchar(255) not null, -- URL to the cover image
    created_by varchar(64) not null REFERENCES users(email), -- store the author of the blog, email or username
    created_at timestamp not null default current_timestamp
);
