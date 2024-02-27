import {del, get, post, put} from "@/utils/http.js";
import {Icon} from "@/types/api.ts";

/**
 *
 * @param name
 * @returns {Promise<*>}
 */
async function getIconList(name?: string): Promise<Icon[]> {
    let params = {};
    if (name && name.length > 0) {
        params = {
            name: name,
        };
    }
    return await get<Icon[]>(`/icon/list`, params);
}

/**
 * 添加图标
 * @param params
 */
async function addIcon(params: Icon): Promise<boolean> {
    return await post<boolean>('/icon', params);
}

/**
 * 添加图标
 * @param params
 */
async function updateIcon(params: Icon): Promise<boolean> {
    return await put<boolean>('/icon', params);
}

/**
 * 添加图标
 * @param id
 */
async function deleteIcons(id: number): Promise<boolean> {
    return await del<boolean>(`/icon/${id}`);
}


export {getIconList, addIcon, updateIcon, deleteIcons};

export type {Icon};