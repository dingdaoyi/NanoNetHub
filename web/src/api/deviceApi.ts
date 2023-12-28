import {del, get, post, put} from "../config/http.ts";
import {PageResult} from "../common/type_def.ts";

// pub id: i32,
//     pub device_code: String,
//     pub product_id: i32,
//     pub parent_id: Option<i32>,
//     pub device_name: Option<String>,
//     /// 设备原数据
//     pub device_info: Option<Json<HashMap<String, String>>>,
interface Device {
    id?: number,
    device_code: string,
    product_id: number,
    parent_id?: number,
    device_name?: string,
    device_info?: Map<string, string>
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

export {deviceDelete, deviceAdd, deviceEdit, devicePage, deviceDetails}

export type {Device}