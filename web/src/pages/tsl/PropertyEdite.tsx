import {Button, Col, Form, Input, InputNumber, Modal, Radio, Row, Select, Space} from 'antd';
import {getDataType, propertyAdd, PropertyType, Unit, unitList} from "../../api/propertyApi.ts";
import React, {useEffect, useState} from "react";
import {MinusCircleOutlined, PlusOutlined} from "@ant-design/icons";

interface PropertyEditeProps {
    visible: boolean;
    updateProperty: (value: PropertyType) => void;
    property: PropertyType;
    onCancel: () => void;
    product_id: number;
}

interface NumberLimit {
    min: number;
    max: number;
}

function PropertyEdite(params: PropertyEditeProps) {
    const property = params.property;
    property.dataType = getDataType(property.data_schema);
    const [form] = Form.useForm();
    const [dataSchemaChildren, setDataSchemaChildren] = useState<React.ReactNode>("")
    const [units, setUnits] = useState<Unit[]>([])
    const [limits, setLimits] = useState<NumberLimit>({min: -2147483648, max: 2147483647})
    useEffect(() => {
        unitList().then(setUnits).catch((err) => {
            console.log(err)
        });
        changeDataSchema(getDataType(property.data_schema));
    }, []);

    /**
     * 提交
     */
    function submitEdite() {
        form
            .validateFields()
            .then((property) => {
                console.log('Received property:',);
                propertyAdd({...property, product_id: params.product_id}).then(() => {
                    form.resetFields();
                    params.onCancel();
                }).catch((err) => {
                    console.log(err)
                })
            })
            .catch((errorInfo) => {
                console.log('Validation failed:', errorInfo);
            });
    }

    const filterOption = (input: string, option?: { label: string, value: string }) =>
        (option?.label ?? '').toLowerCase().includes(input.toLowerCase());

    function changeLimits(value: number) {
        if (value === undefined) return
        switch (value) {
            case 2:
                setLimits({min: -32768, max: 32767})
                break
            default:
                setLimits({
                    min: -2147483648,
                    max: 2147483647
                })
        }

    }

    /**
     * 数据定义
     * @param dataType
     */
    function changeDataSchema(dataType: string) {
        switch (dataType) {
            case "Integer":
                setDataSchemaChildren((
                    <>
                        <Row justify={"space-around"}>
                            <Col span={12}>
                                <Form.Item name={["data_schema", "Integer", "max"]} label="最大值">
                                    <InputNumber style={{width: 120}} precision={0} min={limits.min} max={limits.max}/>
                                </Form.Item>
                            </Col>
                            <Col span={12}>
                                <Form.Item name={["data_schema", "Integer", "min"]} label="最小值">
                                    <InputNumber style={{width: 120}} precision={0} min={limits.min} max={limits.max}/>
                                </Form.Item>
                            </Col>
                        </Row>
                        <Row>
                            <Col span={12}>
                                <Form.Item name={["data_schema", "Integer", "len"]} label="长度">
                                    <Select
                                        style={{width: 120}}
                                        onChange={changeLimits}
                                        options={[
                                            {value: 2, label: '2'},
                                            {value: 4, label: '4'},
                                        ]}
                                    />
                                </Form.Item>
                            </Col>
                            <Col span={12}>
                                <Form.Item name={["data_schema", "Integer", "unit"]} label="单位">
                                    <Select
                                        showSearch
                                        placeholder="搜索选择"
                                        optionFilterProp="children"
                                        filterOption={filterOption}
                                        style={{width: 120}}
                                        options={units.map(unit => ({
                                            value: `${unit.unit}|${unit.unit_name}`, // 组合两个字段作为 value
                                            label: `${unit.unit}|${unit.unit_name}`, // 组合两个字段作为 label
                                        }))}
                                    />
                                </Form.Item>
                            </Col>
                        </Row>
                    </>
                ));
                break
            case "String":
                setDataSchemaChildren((
                    <>
                        <Form.Item name={["data_schema", "Integer", "unit"]} label="单位">
                            <Select
                                showSearch
                                placeholder="搜索选择"
                                optionFilterProp="children"
                                filterOption={filterOption}
                                options={units.map(unit => ({
                                    value: `${unit.unit}|${unit.unit_name}`, // 组合两个字段作为 value
                                    label: `${unit.unit}|${unit.unit_name}`, // 组合两个字段作为 label
                                }))}
                            />
                        </Form.Item>
                    </>));
                break
            case "VaryString":
                setDataSchemaChildren((
                    <Row>
                        <Col span={12}>
                            <Form.Item name={["data_schema", "Integer", "len"]} label="长度">
                                <InputNumber/>
                            </Form.Item>
                        </Col>
                        <Col span={12}>
                            <Form.Item name={["data_schema", "Integer", "unit"]} label="单位">
                                <Select
                                    showSearch
                                    placeholder="搜索选择"
                                    optionFilterProp="children"
                                    filterOption={filterOption}
                                    style={{width: 120}}
                                    options={units.map(unit => ({
                                        value: `${unit.unit}|${unit.unit_name}`, // 组合两个字段作为 value
                                        label: `${unit.unit}|${unit.unit_name}`, // 组合两个字段作为 label
                                    }))}
                                />
                            </Form.Item>
                        </Col>
                    </Row>
                ));
                break
            case "Double":
                setDataSchemaChildren((
                    <>
                        <Row justify={"space-around"}>
                            <Col span={12}>
                                <Form.Item name={["data_schema", "Double", "max"]} label="最大值">
                                    <InputNumber style={{
                                        width: 120
                                    }}/>
                                </Form.Item>
                            </Col>
                            <Col span={12}>
                                <Form.Item name={["data_schema", "Double", "min"]} label="最小值">
                                    <InputNumber style={{
                                        width: 120
                                    }}/>
                                </Form.Item>
                            </Col>
                        </Row>
                        <Form.Item name={["data_schema", "Double", "unit"]} label="单位">
                            <Select
                                showSearch
                                placeholder="搜索选择"
                                optionFilterProp="children"
                                filterOption={filterOption}
                                options={units.map(unit => ({
                                    value: `${unit.unit}|${unit.unit_name}`, // 组合两个字段作为 value
                                    label: `${unit.unit}|${unit.unit_name}`, // 组合两个字段作为 label
                                }))}
                            />
                        </Form.Item>
                    </>
                ));
                break
            case "Float":
                setDataSchemaChildren((
                    <>
                        <Row justify={"space-around"}>
                            <Col span={12}>
                                <Form.Item name={["data_schema", "Float", "max"]} label="最大值">
                                    <InputNumber style={{
                                        width: 120
                                    }}/>
                                </Form.Item>
                            </Col>
                            <Col span={12}>
                                <Form.Item name={["data_schema", "Float", "min"]} label="最小值">
                                    <InputNumber style={{
                                        width: 120
                                    }}/>
                                </Form.Item>
                            </Col>
                        </Row>
                        <Form.Item name={["data_schema", "Float", "unit"]} label="单位">
                            <Select
                                showSearch
                                placeholder="搜索选择"
                                optionFilterProp="children"
                                filterOption={filterOption}
                                options={units.map(unit => ({
                                    value: `${unit.unit}|${unit.unit_name}`, // 组合两个字段作为 value
                                    label: `${unit.unit}|${unit.unit_name}`, // 组合两个字段作为 label
                                }))}
                            />
                        </Form.Item>
                    </>
                ));
                break
            case "Boolean":
                setDataSchemaChildren((
                    <>
                        <Row justify={"space-around"}>
                            <Col span={12}>
                                <Form.Item name={["data_schema", "Boolean", "bool_false"]} label="false">
                                    <Input/>
                                </Form.Item>
                            </Col>
                            <Col span={12}>
                                <Form.Item name={["data_schema", "Boolean", "bool_true"]} label="true">
                                    <Input/>
                                </Form.Item>
                            </Col>
                        </Row>
                    </>
                ));
                break
            case "DateTime":
                setDataSchemaChildren((
                    <>
                        <Form.Item name={["data_schema"]} initialValue={"DateTime"} hidden>
                        </Form.Item>
                    </>
                ));
                break
            case "Enum":
                setDataSchemaChildren((
                    <>
                        <Form.List name={["data_schema", "Enum"]}>
                            {(fields, {add, remove}) => (
                                <>
                                    {fields.map(({key, name, ...restField}) => (
                                        <Space key={key} style={{display: 'flex', marginBottom: 8}} align="baseline">
                                            <Form.Item
                                                {...restField}
                                                name={[name, 'key']}
                                                rules={[{required: true, message: '枚举名称不能为空'}]}
                                            >
                                                <InputNumber precision={0} placeholder="数字类型"/>
                                            </Form.Item>
                                            <Form.Item
                                                {...restField}
                                                name={[name, 'value']}
                                                rules={[{required: true, message: '枚举名称不能为空'}]}
                                            >
                                                <Input placeholder="枚举值"/>
                                            </Form.Item>
                                            <MinusCircleOutlined onClick={() => remove(name)}/>
                                        </Space>
                                    ))}
                                    <Form.Item>
                                        <Button type="dashed" onClick={() => add()} block icon={<PlusOutlined/>}>
                                            添加字段
                                        </Button>
                                    </Form.Item>
                                </>
                            )}
                        </Form.List>
                    </>
                ));
                break
            default:
                setDataSchemaChildren("测试");
        }
    }


    return (
        <>
            <Modal title={property?.property_id ? "编辑属性" : "新增属性"}
                   destroyOnClose={true}
                   open={params.visible}
                   maskClosable={false}  // 阻止点击蒙层关闭
                   onOk={submitEdite}
                   onCancel={() => {
                       form.resetFields()
                       params.onCancel()
                   }}>
                {/* 这里可以放置编辑表单 */}
                <Form form={form}
                      initialValues={{...property}}
                >
                    <Form.Item name="property_name" label="属性名称">
                        <Input/>
                    </Form.Item>
                    <Form.Item name="identifier" label="属性标识">
                        <Input/>
                    </Form.Item>
                    <Form.Item name="description" label="属性描述">
                        <Input/>
                    </Form.Item>
                    <Form.Item name="dataType" label="数据类型">
                        <Radio.Group onChange={(event) => changeDataSchema(event.target.value)}>
                            <Radio value="Integer">数字</Radio>
                            <Radio value={"VaryString"}>定长字符串</Radio>
                            <Radio value={"String"}>变长字符串</Radio>
                            <Radio value={"Enum"}>枚举</Radio>
                            <Radio value={"Boolean"}>布尔</Radio>
                            <Radio value={"Double"}>Double</Radio>
                            <Radio value={"Float"}>Float</Radio>
                            <Radio value={"DateTime"}>DateTime</Radio>
                        </Radio.Group>
                    </Form.Item>
                    {dataSchemaChildren}
                </Form>
            </Modal>
        </>
    );
}

export default PropertyEdite;