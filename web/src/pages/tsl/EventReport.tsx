import {ColumnsType} from "antd/es/table";
import {Button, Input, message, Space, Table} from "antd";
import {PlusOutlined, SearchOutlined} from "@ant-design/icons";
import {listService, Service} from "../../api/serviceApi.ts";
import {useEffect, useState} from "react";

interface EventReportProps {
    productId: number;
}

function EventReport(params: EventReportProps) {
    const [search_param, setSearch_param] = useState<string>('')
    const [editModalVisible, setEditModalVisible] = useState<boolean>(false)
    const [loading, setLoading] = useState(false)
    const [service, setService] = useState<Service>()
    const [serviceList, setServiceList] = useState<Service[]>([])
    const [messageApi, contextHolder] = message.useMessage();

    useEffect(() => {
        setLoading(true)
        listService({
            product_id: params.productId,
            service_types: ["EventReport"],
            search_param
        }).then((res) => {
            setServiceList(res)
            setLoading(false)
        }).catch(async (e) => {
            await messageApi.error(e.message)
        })
    }, []);

    function editeService(value?: Service) {
        setService(value)
        setEditModalVisible(true)
    }

    function deleteService(value: Service) {
        console.log("deleteService", value)
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

    function handleSearch() {
        console.log("handleSearch", search_param)
    }

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
                onCancel={() => setEditModalVisible(false)}
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
    console.log("ServiceEdite", params)
    return (
        <>
            <div>编辑</div>
        </>
    );
}

export default EventReport;