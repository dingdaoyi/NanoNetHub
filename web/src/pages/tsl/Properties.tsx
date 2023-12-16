import {ColumnsType} from "antd/es/table";
import {Button, Space, Table} from "antd";
import {getDataType, getDefinition, propertiesList, PropertyType} from "../../api/propertyApi.ts";
import {useEffect, useState} from "react";

interface PropertiesProps {
    productId: number;
}

function Properties(params: PropertiesProps) {
    const [listData, setListData] = useState<PropertyType[]>([]);
    const [loading, setLoading] = useState(false)
    const [pageParams, setPageParams] = useState({page: 1, size: 10, total: 0});

    const handlePageChange = (page: number, size: number) => {
        setPageParams({total: pageParams.total, page, size});
    };


    // 在组件挂载或搜索参数、分页参数改变时触发请求
    const fetchData = async () => {
        try {
            const data = await propertiesList(params.productId);
            setListData(data);
        } catch (error) {
            // 处理请求错误
            console.error(error)
        }
    };

    useEffect(() => {
        setLoading(true);
        fetchData().then(() => setLoading(false));
    }, [pageParams]);

    const columns: ColumnsType<PropertyType> = [
        {
            title: '属性ID',
            dataIndex: 'property_id',
            key: 'property_id',
        },
        {
            title: '属性名称',
            dataIndex: 'property_name',
            key: 'property_name',
        },
        {
            title: '属性标识',
            dataIndex: 'identifier',
            key: 'identifier',
        },
        {
            title: '数据类型',
            dataIndex: 'dataSchema',
            key: 'dataSchema',
            render: (_, record) => {
                const dataSchema = record.data_schema;
                return getDataType(dataSchema);
            }
        },
        {
            title: '数据定义',
            dataIndex: 'dataSchema',
            key: 'dataSchema',
            render: (_, record) => {
                const dataSchema = record.data_schema;
                return getDefinition(dataSchema);
            }
        },
        {
            title: '操作',
            key: 'action',
            render: (_, value) => {
                console.log(value);
                return (
                    <Space size="middle">
                        <Button type="primary">查看详情</Button>
                        <Button type="primary">编辑</Button>
                        <Button type="primary">删除</Button>
                    </Space>
                )
            },
        },
    ];
    return (
        <>
            <Table columns={columns}
                   dataSource={listData}
                   loading={loading}
                   rowKey="property_id"
                   pagination={{
                       current: pageParams.page,
                       pageSize: pageParams.size,
                       total: pageParams.total, // 如果有后端分页，请使用真实的总数
                       onChange: handlePageChange,
                   }}
            />
        </>
    );
}

export default Properties;