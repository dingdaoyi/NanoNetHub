interface Device {
    id?: number,
    device_code: string,
    product_id: number,
    parent_id?: number,
    device_name?: string,
    device_info?: Map<string, string>
}

interface DeviceLogQuery {
    device_id: number,
    identifier: string,
    timestamp_start: Date,
    timestamp_end: Date
}

interface DeviceShadow {
    device_code: string,
    property_name: string,
    description?: string,
    identifier: string,
    value: object,
    unit: string,
    icon?: string,
    unit_name: string,
    data_type: number,
}

interface DeviceLog {
    create_time: string,
    unit?: string,
    value: object,
    unit_name?: string,
}

interface Icon {
    id?: number,
    name: string,
    icon: string,
    default_icon?: boolean,
}

interface ProductType {
    id?: number,
    create_time?: string,
    deleted?: boolean,
    product_name: string,
    description?: string,
}

interface ProductDict {
    id: number,
    product_name: string,
}

interface PageResult<T> {
    total: number;
    data: T[];
}

interface BaseQuery {
    page: number;
    size: number;
}

interface PropertyType {
    property_id?: number,
    product_id: number,
    identifier: string,
    property_name: string,
    description?: string,
    property_type: string,
    icon?: string,
    data_schema?: DataSchema,
    dataType?: string,  // 用于前端展示
}

interface Unit {
    id: number,
    unit: string,
    unit_name: string,
    unit_description: string,
}

type DataSchema =
    | { Integer: { len: number; unit: string; min: number; max: number; unit_name: string } }
    | { String: { unit: string; unit_name: string } }
    | { VaryString: { len: number; unit: string; unit_name: string } }
    | { Boolean: { bool_false: string; bool_true: string } }
    | { DateTime: null }
    | {
    Enum: Array<{
        key: string;
        value: string;
    }>
}
    | { Double: { len: number; unit: string; min: number; max: number; unit_name: string } };


interface PropertyRef {
    property_id: number,
    serial: number,
}

interface Service {
    service_id?: number,
    product_id: number,
    identifier: string,
    service_name: string,
    service_type: "EventReport" | "Command" | "CommandResponse",
    description: string,
    properties: PropertyRef[],
}

type LoginType = {
    username?: string;
    password?: string;
    remember?: string;
};


export type {
    Device,
    DeviceLogQuery,
    DeviceShadow,
    DeviceLog,
    Icon,
    ProductType,
    ProductDict,
    PageResult,
    BaseQuery,
    Unit,
    DataSchema,
    PropertyType,
    PropertyRef,
    Service,
    LoginType
}
