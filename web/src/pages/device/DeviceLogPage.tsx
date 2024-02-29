import {useEffect, useRef, useState} from "react";
import {device_shadows, DeviceLog, DeviceShadow, listDeviceLog} from "@/api/deviceApi.ts";
import {Avatar, Card, Col, Row, Space, Table} from "antd";
import * as echarts from 'echarts';
import {ColumnsType} from "antd/es/table";

const {Meta} = Card;

interface Params {
    device_id?: number,
    product_id: number,
}

function DeviceLogPage(params: Params) {
    const [shadows, setShadows] = useState<DeviceShadow[]>()
    const [deviceLog, setDeviceLog] = useState<DeviceLog[]>()
    useEffect(() => {
        device_shadows(params.device_id!).then((res) => setShadows(res));
    }, [params.device_id]);
    const dataRef = useRef<HTMLDivElement>(null);
    const [showForm, setShowForm] = useState<boolean>(false); // 控制表单组件的显示状态


    const logColumns: ColumnsType<DeviceLog> = [
        {
            title: '时间',
            dataIndex: 'create_time',
            key: 'create_time',
        },
        {
            title: '采集值',
            dataIndex: 'value',
            key: 'value',
        },
    ];

    async function showDetail(shadow: DeviceShadow) {
        const date = new Date();
        const previousMonth = new Date(date.getFullYear(), date.getMonth() - 2, date.getDate());
        const deviceLogs = await listDeviceLog({
            device_id: params.device_id!,
            identifier: shadow.identifier,
            timestamp_start: previousMonth,
            timestamp_end: date,
        });
        const isNumber = shadow.data_type == 1 || shadow.data_type == 7;
        const chartNumber = () => {
            if (dataRef.current == null) {
                return
            }
            const myChart = echarts.init(dataRef.current);
            myChart.setOption({
                title: {
                    text: shadow.property_name
                },
                xAxis: {
                    type: 'category',
                    name: "时间",
                    data: deviceLogs.map((log) => log.create_time)
                },
                yAxis: {
                    type: 'value',
                    axisLabel: {
                        formatter: '{value}' + shadow.unit
                    }
                },
                series: [
                    {
                        name: '时间',
                        type: 'line',
                        smooth: true,
                        data: deviceLogs.map((log) => log.value)
                    }
                ]
            })
        }
        if (isNumber) {
            setShowForm(false)
            setDeviceLog([])
            setTimeout(() => {
                chartNumber(); // 添加延迟以确保 DOM 元素已经被正确渲染
            }, 0);
        } else {
            setDeviceLog(deviceLogs)
            setShowForm(true)
        }
    }

    return <>
        {shadows &&
            <Row gutter={16}>

                {shadows.map((shadow) => {
                    return <Col key={shadow.identifier} span={6}>
                        <Card
                            onClick={() => showDetail(shadow)}
                        >
                            <Space>
                                {shadow?.icon &&
                                    <Avatar size={"large"} src={<img src={shadow?.icon} alt="暂无图标"/>}/>}
                                <Meta
                                    title={
                                        <Space>
                                            <span>{shadow?.value.toString()}</span>
                                            <span>{shadow?.unit}</span>
                                        </Space>
                                    }
                                    description={shadow?.property_name}
                                />
                            </Space>
                        </Card>
                    </Col>
                })}
                <Col span={24}>
                    {showForm ? <Table columns={logColumns}
                                       dataSource={deviceLog}
                                       rowKey="create_time"
                                       pagination={false}
                    /> : <div ref={dataRef} style={{width: '100%', height: '400px'}}>
                    </div>
                    }
                </Col>

            </Row>
        }
    </>;
}

export default DeviceLogPage;