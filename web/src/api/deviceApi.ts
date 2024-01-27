import {del, get, post, put} from "../config/http.ts";
import {PageResult} from "../common/type_def.ts";
import moment from "moment";


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
    timestamp: string,
    unit?: string,
    value: object,
    unit_name?: string,
}


/**
 * 分页查询设备
 * @param params
 */

async function devicePage(params: {
    device_code?: string,
    device_name?: string,
    product_id?: number,
    parent_id?: number,
    base_query: { page: number, size: number }
}): Promise<PageResult<Device>> {
    return await post<PageResult<Device>>('/device/page', {
        ...params,
        device_code: params.device_code || '',
        device_name: params.device_name || '',
    });
}

/**
 * 编辑设备
 * @param params
 */
async function deviceEdit(params: Device): Promise<boolean> {
    return await put<boolean>('/device', params);
}


/**
 * 添加设备
 * @param params
 */
async function deviceAdd(params: Device): Promise<boolean> {
    return await post<boolean>('/device', params);
}


/**
 * 删除产品
 * @param id
 */
async function deviceDelete(id: number): Promise<boolean> {
    return await del<boolean>(`/device/${id}`);
}

/**
 * 删除产品
 * @param id
 */
async function deviceDetails(id: number): Promise<Device> {
    return await get<Device>(`/device/${id}`);
}


/**
 * 查询一段事件的日志记录
 * @param param
 */
async function deviceLog(param: {
    timestamp_start: string,
    timestamp_end: string,
    device_id: number
}): Promise<DeviceLog[]> {
    return await post<DeviceLog[]>(`/device/log`, param);
}

/**
 * 设备影子
 * @param device_id
 */
async function device_shadows(device_id: number): Promise<DeviceShadow[]> {
    return await get<DeviceShadow[]>(`/device/shadows/${device_id}`);
}

/**
 * 设备影子
 * @param query
 */
async function listDeviceLog(query: DeviceLogQuery): Promise<DeviceLog[]> {
    return await post<DeviceLog[]>("/device/logs", {
        ...query,
        timestamp_start: moment(query.timestamp_start).format("yyyy-MM-DD HH:mm:ss Z"),
        timestamp_end: moment(query.timestamp_end).format("yyyy-MM-DD HH:mm:ss Z"),
    });
}


export {deviceDelete, deviceAdd, deviceEdit, devicePage, deviceDetails, deviceLog, device_shadows, listDeviceLog}

export type {Device, DeviceLog, DeviceShadow, DeviceLogQuery}