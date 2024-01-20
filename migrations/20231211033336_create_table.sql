-- Add migration script here
CREATE TABLE companies
(
    id           INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    company_name varchar(255) NOT NULL
);

CREATE TABLE departments
(
    id              INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    department_name varchar(255) NOT NULL
);

CREATE TABLE roles
(
    id   INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    type varchar(255) NOT NULL
);

CREATE TABLE modules
(
    id   INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    type varchar(255) NOT NULL,
    icon_url varchar(255)
);
-- 子模块 modules从表
CREATE TABLE sub_modules
(
    id        INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    type      varchar(255) NOT NULL,
    module_id INT          NOT NULL,
    FOREIGN KEY (module_id) REFERENCES modules (id)
);
-- 权限关联表
CREATE TABLE permissions
(
    role_id       INT NOT NULL,
    module_id     INT NOT NULL,
    sub_module_id INT NOT NULL,
    PRIMARY KEY (role_id, module_id, sub_module_id),
    FOREIGN KEY (role_id) REFERENCES roles (id),
    FOREIGN KEY (module_id) REFERENCES modules (id),
    FOREIGN KEY (sub_module_id) REFERENCES sub_modules (id)
);

CREATE TABLE posts
(
    id   INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    type varchar(255) NOT NULL
);

CREATE TABLE post_rank
(
    id       INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    rank     INT NOT NULL,
    rank_name varchar(255) NOT NULL
);

CREATE TABLE stores
(
    id         INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    store_name varchar(255) NOT NULL
);

CREATE TABLE users
(
    id           INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    name         varchar(255),
    email        varchar(255),
    phone        varchar(255),
    male         integer,
    birth        varchar(255),
    superior     integer,
    post         integer,
    post_rank    integer,
    company      integer,
    joining_date VARCHAR(255),
    department   integer,
    role         integer,
    FOREIGN KEY (superior) REFERENCES users (id),
    FOREIGN KEY (post) REFERENCES posts (id),
    FOREIGN KEY (post_rank) REFERENCES post_rank (id),
    FOREIGN KEY (company) REFERENCES companies (id),
    FOREIGN KEY (department) REFERENCES departments (id),
    FOREIGN KEY (role) REFERENCES roles (id)
);

CREATE TABLE user_credentials
(
    id       INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    user_id  INT NOT NULL,
    username varchar(255) NOT NULL UNIQUE,
    password varchar(255) NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id)
);