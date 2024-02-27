import {ColumnsType} from "antd/es/table";
import {productAdd, productDelete, productEdit, productPage} from "@/api/productApi.ts";
import {Button, Form, Input, Modal, Space, Table, Typography} from "antd";
import {useEffect, useState} from "react";
import {PlusOutlined, SearchOutlined} from "@ant-design/icons";
import {useNavigate} from "react-router-dom";
import {ProductType} from "@/types/api.ts";


function editProduct(record: ProductType, setEditeType: (value: ProductType | undefined) => void, setEditModalVisible: (value: boolean) => void) {
    console.log("编辑", record)
    setEditeType(record)
    setEditModalVisible(true)
}

async function deleteProduct(id: number | undefined, fetchData: () => Promise<void>) {
    console.log("删除", id)
    Modal.confirm({
        title: '删除产品',
        content: '确定删除该产品吗？',
        okText: '确认',
        cancelText: '取消',
        onOk: async () => {
            await productDelete(id as number)
            await fetchData()
        },
    });
}


function Product() {
    const [listData, setListData] = useState<ProductType[]>([]);
    const [editeProductType, setEditeType] = useState<ProductType>();
    const [product_name, setProduct_name] = useState("");
    const [pageParams, setPageParams] = useState({page: 1, size: 10, total: 0});
    const [loading, setLoading] = useState(false);
    const [editModalVisible, setEditModalVisible] = useState(false); // 新增这一行
    const {Paragraph} = Typography;

    const navigate = useNavigate()
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
            title: 'productKey',
            dataIndex: 'product_key',
            key: 'product_key',
            render: (text) => (
                <>
                    <Paragraph copyable>{text}</Paragraph>
                </>
            ),
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
            width: "280px",
            render: (_, record) => (
                <Space size="middle">
                    <Button type="primary"
                            onClick={() => editProduct(record, setEditeType, setEditModalVisible)}>编辑</Button>
                    <Button type="primary" onClick={() => deleteProduct(record.id, fetchData)}>删除</Button>
                    <Button type="primary" onClick={() => navigate(`/admin/tsl/${record.id}`)}>物模型</Button>
                </Space>
            ),
        },
    ];

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
    const handleEditeSubmit = async () => {
        const productType = editeProductType as ProductType;
        editeProductType?.id ? await productEdit(productType) : await productAdd(productType);
        setEditModalVisible(false);
        fetchData().then(() => setLoading(false))
    };
    const handleAdd = () => {
        editProduct({product_name: "", description: ""}, setEditeType, setEditModalVisible);
    };

    useEffect(() => {
        setLoading(true);
        fetchData().then(() => setLoading(false));
    }, [pageParams]);
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
                        placeholder="产品名称"
                        value={product_name}
                        width="200px"
                        onChange={(e) => setProduct_name(e.target.value)}
                    />
                    <Button type="primary" icon={<SearchOutlined/>} onClick={handleSearch}>搜索</Button>
                </Space>
                <Button type="primary" icon={<PlusOutlined/>} onClick={handleAdd}>新增</Button>
            </Space>
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
            <Modal
                title={editeProductType?.id ? "编辑产品" : "新增产品"}
                open={editModalVisible}
                destroyOnClose={true}
                onCancel={() => setEditModalVisible(false)}
                onOk={async () => {
                    await handleEditeSubmit();
                }}
            >
                {/* 这里可以放置编辑表单 */}
                <Form>
                    <Form.Item label="产品名称">
                        <Input name="product_name" defaultValue={editeProductType?.product_name}
                               onChange={(value) => {
                                   editeProductType!.product_name = value.target.value
                                   setEditeType(editeProductType)
                               }}/>
                    </Form.Item>
                    <Form.Item label="产品描述">
                        <Input name="description" defaultValue={editeProductType?.description}
                               onChange={(value) => {
                                   editeProductType!.description = value.target.value
                                   setEditeType(editeProductType)
                               }}/>
                    </Form.Item>
                </Form>
            </Modal>
        </>
    )
}

export default Product