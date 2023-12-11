-- Add migration script here
CREATE TABLE companies
(
    id   INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    company_name varchar(255),
    company_name_cn varchar(255),
);

CREATE TABLE departments
(
    id   INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    departments_name varchar(255),
    departments_name_cn varchar(255),
);

CREATE TABLE module
(
    id   INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    type varchar(255),
);

CREATE TABLE role
(
    id   INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    type varchar(255),
);

CREATE TABLE users
(
    id   INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    name varchar(255),
    email varchar(255),
    phone varchar(255),
    male integer,
    birth varchar,
);