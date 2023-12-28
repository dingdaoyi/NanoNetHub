import {Link, useParams} from "react-router-dom";
import {Breadcrumb, Space, Tabs, TabsProps} from "antd";
import {useEffect, useState} from "react";
import {Device, deviceDetails} from "../../api/deviceApi.ts";

function DeviceDetails() {
    const {deviceId} = useParams();
    const productIdAsNumber = parseInt(deviceId || "0", 10);
    const [device, setDevice] = useState<Device>()

    useEffect(() => {
        deviceDetails(productIdAsNumber).then((res) => {
            setDevice(res)
        })
    }, []);
    const items: TabsProps['items'] = [
        {
            key: 'details',
            label: '设备详情',
            children: '设备详情',
        },
        {
            key: 'deviceLog',
            label: '设备日志',
            children: '设备日志',
        },
    ];
    return (

        <>
            <Space>
                <Breadcrumb
                    style={{marginBottom: 16,}}
                    items={[
                        {
                            title: <Link to={"/admin/product"}>设备管理</Link>,
                        },
                        {
                            title: '设备详情',
                        },
                        {
                            title: device?.device_code,
                        },
                    ]}
                />
            </Space>
            <Tabs
                defaultActiveKey="1"
                items={items}
            />
        </>
    )
}

export default DeviceDetails