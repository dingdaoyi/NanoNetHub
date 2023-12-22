import {del, post, put} from "../config/http.ts";

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

/**
 * 服务添加
 * @param params
 */
async function addService(params: Service) {
    return await post<boolean>('/service', params)
}

/**
 * 服务编辑
 * @param params
 */
async function updateService(params: Service) {
    return await put<boolean>(`/service/${params.service_id}`, params)
}

/**
 * 删除服务
 * @param id
 */
async function serviceDelete(id: number) {
    return await del<boolean>(`/service/${id}`)
}

/**
 * 添加服务
 * @param params
 */
function listService(params: { service_type?: string, product_id?: number, search_param?: string }) {
    return post<Service[]>('/service/list', params)
}


export {listService, addService, updateService, serviceDelete}

export type {Service, PropertyRef}