import {ColumnsType} from "antd/es/table";
import {productPage, ProductType} from "../../api/productApi.tsx";
import {Button, Input, Space, Table} from "antd";
import {useEffect, useState} from "react";


function editProduct(record: ProductType) {
    console.log("编辑", record)
}

function deleteProduct(id: number | undefined) {
    console.log("删除", id)

}

const columns: ColumnsType<ProductType> = [
    {
        title: '产品名称',
        dataIndex: 'product_name',
        key: 'product_name',
        width: "150px",
    },
    {
        title: '描述',
        dataIndex: 'description',
        key: 'description',
        ellipsis: true,
    },
    {
        title: '创建时间',
        dataIndex: 'create_time',
        key: 'create_time',
        width: "220px",
    },
    {
        title: '操作',
        key: 'action',
        width: "120px",
        render: (_, record) => (
            <Space size="middle">
                <a onClick={() => editProduct(record)}>编辑</a>
                <a onClick={() => deleteProduct(record.id)}>删除</a>
            </Space>
        ),
    },
];


function Product() {
    const [listData, setListData] = useState<ProductType[]>([]);
    const [product_name, setProduct_name] = useState("");
    const [pageParams, setPageParams] = useState({page: 1, size: 10, total: 0});
    const [loading, setLoading] = useState(false);

    // 在组件挂载或搜索参数、分页参数改变时触发请求
    const fetchData = async () => {
        try {
            const data = await productPage({
                base_query: pageParams,
                product_name,
            });
            setListData(data.data);
        } catch (error) {
            // 处理请求错误
            console.error(error)
        }
    };

    const handleSearch = () => {
        setPageParams({...pageParams, page: 1});
    };
    const handlePageChange = (page: number, size: number) => {
        setPageParams({total: pageParams.total, page, size});
    };

    useEffect(() => {
        setLoading(true);
        fetchData().then(() => setLoading(false));
    }, [pageParams]);
    return (
        <>
            <Space align="center" style={{marginBottom: 16}}>
                <Input
                    placeholder="产品名称"
                    value={product_name}
                    width="200px"
                    onChange={(e) => setProduct_name(e.target.value)}
                />
                <Button type="primary" onClick={handleSearch}>搜索</Button> </Space>
            <Table columns={columns}
                   dataSource={listData}
                   loading={loading}
                   rowKey="id"
                   pagination={{
                       current: pageParams.page,
                       pageSize: pageParams.size,
                       total: pageParams.total, // 如果有后端分页，请使用真实的总数
                       onChange: handlePageChange,
                   }}
            />
        </>
    )
}

export default Product