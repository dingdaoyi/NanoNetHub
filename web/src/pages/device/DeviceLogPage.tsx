import {useEffect, useRef, useState} from "react";
import {device_shadows, DeviceShadow, listDeviceLog} from "@/api/deviceApi.ts";
import {Avatar, Card, Col, Row, Space} from "antd";
import * as echarts from 'echarts';

const {Meta} = Card;

interface Params {
    device_id?: number,
    product_id: number,
}

function DeviceLogPage(params: Params) {
    const [shadows, setShadows] = useState<DeviceShadow[]>()
    useEffect(() => {
        device_shadows(params.device_id!).then((res) => setShadows(res));
    }, [params]);
    const echartsRef = useRef<HTMLDivElement>(null);

    async function showDetail(shadow: DeviceShadow) {
        const date = new Date();
        const previousMonth = new Date(date.getFullYear(), date.getMonth() - 2, date.getDate());
        const deviceLogs = await listDeviceLog({
            device_id: params.device_id!,
            identifier: shadow.identifier,
            timestamp_start: previousMonth,
            timestamp_end: date,
        });
        console.log(deviceLogs)

        const chartNumber = () => {
            const myChart = echarts.init(echartsRef.current)
            const isNumber = shadow.data_type == 1 || shadow.data_type == 7;
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
                        type: isNumber ? 'line' : 'bar',
                        smooth: true,
                        data: deviceLogs.map((log) => log.value)
                    }
                ]
            })
        }
        chartNumber()
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
                    <div ref={echartsRef} style={{width: '100%', height: '400px'}}/>
                </Col>
            </Row>
        }
    </>;
}

export default DeviceLogPage;