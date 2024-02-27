import {ColumnsType} from "antd/es/table";
import {Button, Form, Input, InputNumber, message, Modal, Select, Space, Table} from "antd";
import {MinusCircleOutlined, PlusOutlined, SearchOutlined} from "@ant-design/icons";
import {addService, listService, serviceDelete, updateService} from "@/api/serviceApi.ts";
import {useEffect, useState} from "react";
import {propertiesList} from "@/api/propertyApi.ts";
import {PropertyType, Service} from "@/types/api.ts";

interface EventReportProps {
    productId: number;
}

function TslService(params: EventReportProps) {
    const [search_param, setSearch_param] = useState<string>('')
    const [serviceType, setServiceType] = useState<string>()
    const [editModalVisible, setEditModalVisible] = useState<boolean>(false)
    const [loading, setLoading] = useState(false)
    const [service, setService] = useState<Service>()
    const [serviceList, setServiceList] = useState<Service[]>([])
    const [messageApi, contextHolder] = message.useMessage();

    function loadData() {
        setLoading(true)
        listService({
            product_id: params.productId,
            service_type: serviceType,
            search_param
        }).then((res) => {
            setServiceList(res)
            setLoading(false)
        }).catch(async (e) => {
            await messageApi.error(e.message)
        })
    }

    useEffect(loadData, [serviceType]);

    function handleSearch() {
        loadData();
    }

    function editeService(value?: Service) {
        setService(value)
        setEditModalVisible(true)
    }

    function deleteService(value: Service) {
        console.log("deleteService", value)
        Modal.confirm({
            title: '删除服务',
            content: '确定删除该服务吗？',
            okText: '确认',
            cancelText: '取消',
            onOk: async () => {
                await serviceDelete(value.service_id as number)
                loadData()
            },
        })
    }

    const columns: ColumnsType<Service> = [
        {
            title: '服务id',
            dataIndex: 'service_id',
            key: 'service_id',
            width: 100,
        },
        {
            title: '服务名称',
            dataIndex: 'service_name',
            key: 'service_name',
            width: 180,
        },
        {
            title: '服务标识',
            dataIndex: 'identifier',
            key: 'identifier',
            width: 180,
        },
        {
            title: '服务类型',
            dataIndex: 'service_type',
            key: 'service_type',
            width: 180,
            render(_, service) {
                switch (service.service_type) {
                    case "EventReport":
                        return "事件上报"
                    case "CommandResponse":
                        return "指令回复"
                    case "Command":
                        return "指令下发"
                }
            }
        },
        {
            title: '服务描述',
            dataIndex: 'description',
            key: 'description',
        },
        {
            title: '操作',
            key: 'action',
            render: (_, value) => {
                return (
                    <Space size="middle">
                        <Button type="primary" onClick={() => editeService(value)}>编辑</Button>
                        <Button type="primary" onClick={() => deleteService(value)}>删除</Button>
                    </Space>
                )
            },
        },
    ];


    return (
        <>
            {contextHolder}
            <Space align="center" style={{
                marginBottom: 16,
                display: 'flex',
                justifyContent: 'space-between',
                alignItems: 'center'
            }}>
                <Space>
                    <Select
                        style={{width: 120}}
                        virtual={false}
                        placeholder={"服务类型"}
                        onChange={(value) => {
                            setServiceType(value)
                            loadData()
                        }}
                        onClear={() => {
                            setServiceType(undefined)
                        }}
                        allowClear
                        options={[
                            {value: 'EventReport', label: '事件上报'},
                            {value: 'Command', label: '指令下发'},
                            {value: 'CommandResponse', label: '指令回复'},
                        ]}
                    />
                    <Input
                        placeholder="服务名称、服务标识"
                        value={search_param}
                        width="450px"
                        onChange={(e) => setSearch_param(e.target.value)}
                    />
                    <Button type="primary" icon={<SearchOutlined/>} onClick={handleSearch}>搜索</Button>
                </Space>
                <Button type="primary" icon={<PlusOutlined/>} onClick={() => editeService()}>新增</Button>
            </Space>
            <Table columns={columns}
                   dataSource={serviceList}
                   loading={loading}
                   rowKey="service_id"
                   pagination={false}
            />
            {editModalVisible && <ServiceEdite
                onCancel={() => {
                    setEditModalVisible(false)
                    handleSearch()
                }}
                visible={editModalVisible}
                service={service}
                product_id={params.productId}/>}
        </>
    );

}

interface ServiceEditeProps {
    visible: boolean;
    onCancel: () => void;
    service?: Service;
    product_id: number;

}

function ServiceEdite(params: ServiceEditeProps) {
    const service = params.service;
    const [form] = Form.useForm();
    const [loading, setLoading] = useState(false)
    const [serviceType, setServiceType] = useState<"EventReport" | "Command" | "CommandResponse">()
    useEffect(() => {
        if (params.service?.service_type) {
            setServiceType(params.service!.service_type)
        }
    }, [params.service]);


    function submitEdite() {
        setLoading(true)
        form.validateFields().then(async (values) => {
            const serviceParam = {
                ...values,
                service_id: service?.service_id,
                product_id: params.product_id,
                properties: values.properties || []
            };
            const res = serviceParam.service_id ? await updateService(serviceParam) : await addService(serviceParam)
            setLoading(false)
            if (res) {
                params.onCancel();
            }
        }).catch((e) => {
                console.log(e)
                setLoading(false)
            }
        )
    }

    return (
        <>
            <Modal title={service?.service_id ? "编辑服务" : "新增服务"}
                   destroyOnClose={true}
                   open={params.visible}
                   maskClosable={false}  // 阻止点击蒙层关闭
                   onOk={submitEdite}
                   confirmLoading={loading}
                   onCancel={params.onCancel}>
                {/* 这里可以放置编辑表单 */}
                <Form form={form}
                      initialValues={{...service}}
                >
                    <Form.Item name="service_name" label="服务名称"
                               rules={[{required: true, message: '服务名称不能为空'}]}
                    >
                        <Input/>
                    </Form.Item>
                    <Form.Item name="identifier" label="服务标识"
                               rules={[{required: true, message: '服务标识不能为空'}]}
                    >
                        <Input/>
                    </Form.Item>
                    <Form.Item name="description" label="服务描述">
                        <Input/>
                    </Form.Item>
                    <Form.Item name="service_type" label="服务类型"
                               rules={[{required: true, message: '服务类型不能为空'}]}
                    >
                        <Select
                            style={{width: 120}}
                            virtual={false}
                            allowClear
                            onChange={(value) => {
                                setServiceType(value)
                            }}
                            options={[
                                {value: 'EventReport', label: '事件上报'},
                                {value: 'Command', label: '指令下发'},
                                {value: 'CommandResponse', label: '指令回复'},
                            ]}
                        />
                    </Form.Item>
                    {
                        serviceType &&
                        <ServiceEditeCustom serviceType={serviceType} product_id={params.product_id}/>
                    }
                </Form>

            </Modal>
        </>
    );
}

function ServiceEditeCustom(params: {
    serviceType: "EventReport" | "Command" | "CommandResponse",
    product_id: number,
}) {
    const [properties, setProperties] = useState<PropertyType[]>([])
    const [customForm, setCustomForm] = useState(false)
    const [serviceCommands, setServiceCommands] = useState<Service[]>([])
    useEffect(() => {
        propertiesList(params.product_id)
            .then((res) => {
                setProperties(res)
            })
            .catch((e) => {
                console.log(e)
            });
        listService({
            service_type: "Command",
            product_id: params.product_id,
            search_param: ''
        }).then((res) => {
            setServiceCommands(res)
        })

    }, []);
    useEffect(() => {
        if (params.serviceType === "CommandResponse") {
            setCustomForm(true);
        }
    }, [params.serviceType]);
    const filterOption = (input: string, option?: { label: string, value: number }) =>
        (option?.label ?? '').toLowerCase().includes(input.toLowerCase());
    return (
        <>
            {customForm && <Form.Item name="command_id" label="对应指令名称">
                <Select
                    style={{width: 120}}
                    options={serviceCommands.map(svc => ({
                        value: svc.service_id!,
                        label: svc.service_name
                    }))}
                />
            </Form.Item>}
            <Form.Item label="属性选择">
                <Form.List name={["properties"]}>
                    {(fields, {add, remove}) => (
                        <>
                            {fields.map(({key, name, ...restField}) => (
                                <Space key={key} style={{display: 'flex', marginBottom: 8}} align="baseline">
                                    <Form.Item
                                        {...restField}
                                        name={[name, 'serial']}
                                        initialValue={fields.length}
                                        rules={[{required: true, message: '序号不能为空'}]}
                                    >
                                        <InputNumber precision={0} disabled/>
                                    </Form.Item>
                                    <Form.Item
                                        {...restField}
                                        name={[name, 'property_id']}
                                        rules={[{required: true, message: '枚举名称不能为空'}]}
                                    >
                                        <Select
                                            showSearch
                                            placeholder="搜索选择"
                                            optionFilterProp="children"
                                            filterOption={filterOption}
                                            style={{width: 120}}
                                            options={properties.map(property => ({
                                                value: property.property_id!, // 组合两个字段作为 value
                                                label: property.property_name, // 组合两个字段作为 label
                                            }))}
                                        />
                                    </Form.Item>
                                    <MinusCircleOutlined onClick={() => remove(name)}/>
                                </Space>
                            ))}
                            <Form.Item>
                                <Button type="dashed" onClick={() => add()} block icon={<PlusOutlined/>}>
                                    添加属性
                                </Button>
                            </Form.Item>
                        </>
                    )}
                </Form.List>
            </Form.Item>
        </>
    )
}

export default TslService;