import {ColumnsType} from "antd/es/table";
import {Button, Input, Modal, Space, Table} from "antd";
import {getDataType, getDefinition, propertiesList, propertyDelete, PropertyType} from "../../api/propertyApi.ts";
import {useEffect, useState} from "react";
import {PlusOutlined, SearchOutlined} from "@ant-design/icons";
import PropertyEdite from "./PropertyEdite.tsx";

interface PropertiesProps {
    productId: number;
}

function Properties(params: PropertiesProps) {
    const [listData, setListData] = useState<PropertyType[]>([]);
    const [loading, setLoading] = useState(false)
    const [search_param, setSearch_param] = useState('')
    const [editModalVisible, setEditModalVisible] = useState(false); // 新增这一行
    const defaultProperty = {
        product_id: params.productId,
        property_name: '',
        identifier: '',
    };
    const [editeProperty, setEditeProperty] = useState<PropertyType>(defaultProperty)
    // 在组件挂载或搜索参数、分页参数改变时触发请求
    const fetchData = async () => {
        try {
            const data = await propertiesList(params.productId, search_param);
            setListData(data);
        } catch (error) {
            // 处理请求错误
            console.error(error)
        }
    };

    const handleSearch = () => {
        setLoading(true);
        fetchData().then(() => setLoading(false));
    };

    const handleAdd = () => {
        setEditModalVisible(true)
    };

    const startEditeProperty = (value: PropertyType) => {
        setEditeProperty(value)
        setEditModalVisible(true)
    }
    useEffect(() => {
        handleSearch()
    }, []);

    /**
     * 删除
     * @param value
     */
    function deleteProperty(value: PropertyType) {
        Modal.confirm({
            title: '删除属性',
            content: '确定删除该属性吗？',
            okText: '确认',
            cancelText: '取消',
            centered: true,
            onOk: async () => {
                setLoading(true);
                propertyDelete(value.property_id!).then(() => {
                    setLoading(false);
                    handleSearch()
                });
            },
        });
    }

    const columns: ColumnsType<PropertyType> = [
        {
            title: '属性ID',
            dataIndex: 'property_id',
            key: 'property_id',
            width: 100,
        },
        {
            title: '属性名称',
            dataIndex: 'property_name',
            key: 'property_name',
            width: 180,
        },
        {
            title: '属性标识',
            dataIndex: 'identifier',
            key: 'identifier',
            width: 180,
        },
        {
            title: '数据类型',
            dataIndex: 'dataSchema',
            key: 'dataSchema',
            width: 180,
            render: (_, record) => {
                const dataSchema = record.data_schema;
                return getDataType(dataSchema);
            }
        },
        {
            title: '数据定义',
            dataIndex: 'dataSchema',
            key: 'dataSchema',
            ellipsis: true,
            render: (_, record) => {
                // 设置最大宽度和超过宽度时的样式
                const dataSchema = record.data_schema;
                return getDefinition(dataSchema);
            }
        },
        {
            title: '操作',
            key: 'action',
            render: (_, value) => {
                return (
                    <Space size="middle">
                        <Button type="primary" onClick={() => startEditeProperty(value)}>编辑</Button>
                        <Button type="primary" onClick={() => deleteProperty(value)}>删除</Button>
                    </Space>
                )
            },
        },
    ];
    return (
        <>
            <Space align="center" style={{
                marginBottom: 16,
                display: 'flex',
                justifyContent: 'space-between',
                alignItems: 'center'
            }}>
                <Space>
                    <Input
                        placeholder="属性名称、属性标识"
                        value={search_param}
                        width="450px"
                        onChange={(e) => setSearch_param(e.target.value)}
                    />
                    <Button type="primary" icon={<SearchOutlined/>} onClick={handleSearch}>搜索</Button>
                </Space>
                <Button type="primary" icon={<PlusOutlined/>} onClick={handleAdd}>新增</Button>
            </Space>
            <Table columns={columns}
                   dataSource={listData}
                   loading={loading}
                   rowKey="property_id"
                   pagination={false}
            />
            {editModalVisible && <PropertyEdite
                visible={editModalVisible}
                onCancel={() => {
                    setEditeProperty(defaultProperty)
                    setEditModalVisible(false)
                    handleSearch()
                }}
                updateProperty={setEditeProperty}
                property={editeProperty}
                product_id={params.productId}
            />}
        </>
    );
}

export default Properties;