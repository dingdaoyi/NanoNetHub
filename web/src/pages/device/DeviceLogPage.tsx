import {useEffect, useState} from "react";
import {device_shadows, DeviceShadow, listDeviceLog} from "../../api/deviceApi.ts";
import {Avatar, Card, Col, Row, Space} from "antd";

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
    }

    return (<>
        {shadows &&
            <Row gutter={16}>

                {shadows.map((shadow) => {
                    return (
                        <Col key={shadow.identifier} span={6}>
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
                    )
                })}
            </Row>
        }
    </>);
}

export default DeviceLogPage;