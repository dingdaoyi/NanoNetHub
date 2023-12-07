-- Add migration script here
CREATE TABLE tb_product
(
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    tenant_id    INTEGER     NOT NULL DEFAULT -1,
    create_time  DATETIME    NOT NULL DEFAULT '1000-01-01 00:00:00',
    deleted      INTEGER     NOT NULL DEFAULT -1,
    product_name VARCHAR(50) NOT NULL,
    description  VARCHAR(50)
);
-- 属性表
CREATE TABLE tb_property
(
    property_id   INTEGER PRIMARY KEY AUTOINCREMENT,
    identifier    TEXT        NOT NULL,
    property_name TEXT        NOT NULL,
    description   TEXT,
    data_type     VARCHAR(50) NOT NULL,
    data_schema   TEXT        NOT NULL
);

-- 服务表
CREATE TABLE tb_service
(
    service_id   INTEGER PRIMARY KEY AUTOINCREMENT,
    identifier   TEXT NOT NULL,
    service_name TEXT NOT NULL,
    service_type TEXT NOT NULL,
    description  TEXT,
    properties   TEXT NOT NULL
);

-- 服务和属性关联表,
CREATE TABLE tb_service_property
(
    service_id  INTEGER NOT NULL,
    property_id INTEGER NOT NULL,
    serial      INTEGER NOT NULL,
    FOREIGN KEY (service_id) REFERENCES tb_service (service_id),
    FOREIGN KEY (property_id) REFERENCES tb_property (property_id),
    UNIQUE (service_id, serial)
);




