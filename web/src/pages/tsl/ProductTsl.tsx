import {Link, useParams} from "react-router-dom";
import {Breadcrumb, Space, Tabs, TabsProps} from "antd";
import Properties from "./Properties.tsx";

function ProductTsl() {
    const {productId} = useParams();
    const productIdAsNumber = parseInt(productId || "0", 10);

    console.log("物模型", productIdAsNumber)

    const items: TabsProps['items'] = [
        {
            key: 'property',
            label: '属性',
            children: <Properties productId={productIdAsNumber}/>,
        },
        {
            key: 'event',
            label: '事件',
            children: '事件',
        },
        {
            key: 'service',
            label: '服务',
            children: '服务',
        },
        {
            key: 'TSL',
            label: 'TSL查看',
            children: 'TSL查看',
        },
    ];
    return (

        <>
            <Space>
                <Breadcrumb
                    style={{marginBottom: 16,}}
                    items={[
                        {
                            title: <Link to={"/admin/product"}>产品管理</Link>,
                        },
                        {
                            title: '物模型',
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

export default ProductTsl