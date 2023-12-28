-- Add migration script here


-- 服务和属性关联表,
CREATE TABLE tb_device
(
--     设备编号
    id          INTEGER primary key AUTOINCREMENT,
--     设备编号
    device_code text    NOT NULL,
--     产品id
    product_id  INTEGER NOT NULL,
--     父级id
    parent_id   INTEGER,
--     设备名称
    device_name text,
--     设备详细信息
    device_info text,
    FOREIGN KEY (product_id) REFERENCES tb_product (id),
    FOREIGN KEY (parent_id) REFERENCES tb_device (id),
    UNIQUE (device_code)
);
