import {Link, useParams} from "react-router-dom";
import {Breadcrumb, Space, Tabs, TabsProps} from "antd";
import Properties from "./Properties.tsx";
import EventReport from "./EventReport.tsx";
import {useEffect, useState} from "react";
import {productDetails, ProductType} from "../../api/productApi.ts";

function ProductTsl() {
    const {productId} = useParams();
    const productIdAsNumber = parseInt(productId || "0", 10);
    const [product, setProduct] = useState<ProductType>()

    useEffect(() => {
        productDetails(productIdAsNumber).then((res) => {
            setProduct(res)
        })
    }, []);
    const items: TabsProps['items'] = [
        {
            key: 'property',
            label: '属性',
            children: <Properties productId={productIdAsNumber}/>,
        },
        {
            key: 'event',
            label: '事件',
            children: <EventReport productId={productIdAsNumber}/>
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
                        {
                            title: product?.product_name,
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