import {useEffect, useState} from "react";
import {useNavigate} from "react-router-dom";
import {ColumnsType} from "antd/es/table";
import {Button, Form, Input, Modal, Select, Space, Table} from "antd";
import {PlusOutlined, SearchOutlined} from "@ant-design/icons";
import {Device, deviceAdd, deviceDelete, deviceEdit, devicePage} from "@/api/deviceApi.ts";
import {productDict} from "@/api/productApi.ts";
import {ProductDict} from "@/types/api.ts";

function DeviceInfo() {

    const [listData, setListData] = useState<Device[]>([]);
    const [editeDevice, setEditeDevice] = useState<Device>();
    const [deviceCode, setDeviceCode] = useState("");
    const [pageParams, setPageParams] = useState({page: 1, size: 10});
    const [pageTotal, setPageTotal] = useState(0)
    const [loading, setLoading] = useState(false);
    const [editModalVisible, setEditModalVisible] = useState(false);
    const [productList, setProductList] = useState<ProductDict[]>([])

    const navigate = useNavigate()


    function handleEditDevice(record?: Device) {
        setEditeDevice(record)
        setEditModalVisible(true)
    }

    async function deleteDevice(id: number) {
        Modal.confirm({
            title: '删除设备',
            content: '确定删除该设备吗？',
            okText: '确认',
            cancelText: '取消',
            onOk: async () => {
                await deviceDelete(id as number)
                await fetchData()
            },
        });
    }

    const columns: ColumnsType<Device> = [
        {
            title: '设备编号',
            dataIndex: 'device_code',
            key: 'device_code',
            width: "150px",
        },
        {
            title: '设备名称',
            dataIndex: 'device_name',
            key: 'device_name',
            ellipsis: true,
        },
        {
            title: '产品名称',
            dataIndex: 'product_id',
            key: 'product_id',
            render: (_, record) => (
                <Space size="middle">
                    {productList.find(item => item.id === record.product_id)?.product_name || ""}
                </Space>
            ),
        },
        {
            title: '是否为子设备',
            dataIndex: 'parent_id',
            key: 'parent_id',
            render: (_, record) => (
                record.parent_id ? "是" : "否"
            ),
        },
        {
            title: '操作',
            key: 'action',
            width: "280px",
            render: (_, record) => (
                <Space size="middle">
                    <Button type="primary"
                            onClick={() => handleEditDevice(record)}>编辑</Button>
                    <Button type="primary" onClick={() => deleteDevice(record.id!)}>删除</Button>
                    <Button type="primary"
                            onClick={() => navigate(`/admin/device/details/${record.id}`)}>设备详情</Button>
                </Space>
            ),
        },
    ];

    const fetchData = async () => {
        try {
            const data = await devicePage({
                base_query: pageParams,
                device_code: deviceCode,
            });
            setListData(data.data);
            setPageTotal(data.total);
        } catch (error) {
            // 处理请求错误
            console.error(error)
        }
    };

    const handleSearch = () => {
        setPageParams({...pageParams, page: 1});
    };
    const handlePageChange = (page: number, size: number) => {
        setPageParams({page, size});
    };

    const handleAdd = () => {
        handleEditDevice()
    };

    useEffect(() => {
        setLoading(true);
        fetchData().then(() => setLoading(false));
    }, [pageParams]);
    useEffect(() => {
        productDict().then(res => setProductList(res))
    }, []);
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
                        placeholder="设备编号"
                        value={deviceCode}
                        width="200px"
                        onChange={(e) => setDeviceCode(e.target.value)}
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
                       total: pageTotal, // 如果有后端分页，请使用真实的总数
                       onChange: handlePageChange,
                   }}
            />
            {editModalVisible && <DeviceEdite
                editeDevice={editeDevice}
                editModalVisible={editModalVisible}
                onCancel={() => {
                    setEditModalVisible(false)
                    handleSearch()
                }}
                products={productList}
            />
            }
        </>
    )
}

interface DeviceEditeProps {
    editeDevice?: Device
    editModalVisible: boolean
    onCancel: () => void
    products: ProductDict[],

}

function DeviceEdite(params: DeviceEditeProps) {
    const [form] = Form.useForm();
    const editeDevice = params.editeDevice;
    const handleEditeSubmit = async () => {
        form
            .validateFields()
            .then(async (device) => {
                const res = editeDevice?.id ? await deviceEdit({...editeDevice, ...device}) :
                    await deviceAdd({...editeDevice, ...device});
                if (res) {
                    form.resetFields()
                    params.onCancel()
                }
            })
            .catch((errorInfo) => {
                console.log('Validation failed:', errorInfo);
            });
    };
    const filterOption = (input: string, option?: { label: string, value: number }) =>
        (option?.label ?? '').toLowerCase().includes(input.toLowerCase());

    return (
        <>
            <Modal
                title={editeDevice?.id ? "编辑设备" : "新增设备"}
                open={params.editModalVisible}
                destroyOnClose={true}
                maskClosable={false}
                onCancel={() => {
                    form.resetFields();
                    params.onCancel()
                }}
                onOk={async () => {
                    await handleEditeSubmit();
                }}
            >
                <Form form={form} initialValues={{...editeDevice}}>
                    <Form.Item name="device_code" label="设备编号"
                               rules={[{
                                   required: true,
                                   message: "请输入设备编号"
                               }, {
                                   pattern: /^[a-zA-Z0-9_]{6,32}$/,
                                   message: "设备编号只能由数字,字母,下划线组成,长度6-32位"
                               }]}>
                        <Input/>
                    </Form.Item>
                    <Form.Item name="product_id" label="产品" rules={[
                        {
                            required: true,
                            message: '请选择产品',
                        },
                    ]}>
                        <Select
                            showSearch
                            placeholder="搜索选择"
                            optionFilterProp="children"
                            filterOption={filterOption}
                            options={params.products.map(product => ({
                                value: product.id,
                                label: product.product_name,
                            }))}
                        />
                    </Form.Item>
                    <Form.Item name="device_name" label="设备名称">
                        <Input/>
                    </Form.Item>
                </Form>
            </Modal>
        </>
    )
}

export default DeviceInfo
