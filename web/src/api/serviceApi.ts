import {post} from "../config/http.ts";

interface Service {
    service_id?: number,
    product_id: number,
    identifier: string,
    service_name: string,
    service_type: "EventReport" | "Command" | "CommandResponse",
    description: string,
    properties: number[],
}

/**
 * 获取服务列表
 * @param params
 */
function listService(params: { service_types: string[], product_id?: number, search_param?: string }) {
    return post<Service[]>('/service/list', params)
}

export {listService}
export type {Service}