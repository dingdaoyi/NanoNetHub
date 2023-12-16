import {del, post, put} from "../config/http.ts";
import {PageResult} from "../common/type_def.ts";

interface ProductType {
    id?: number,
    create_time?: string,
    deleted?: boolean,
    product_name: string,
    description?: string,
}


/**
 * 分页查询产品
 * @param params
 */
async function productPage(params: {
    product_name?: string,
    base_query: { page: number, size: number }
}): Promise<PageResult<ProductType[]>> {
    return await post<PageResult<ProductType[]>>('/product/page', params);
}

/**
 * 修改产品
 * @param params
 */
async function productEdit(params: ProductType): Promise<void> {
    console.log("product edit", params)
    await put<void>('/product', params);
}

/**
 * 添加产品
 * @param params
 */
async function productAdd(params: ProductType): Promise<void> {
    console.log("product edit", params)
    await post<void>('/product', params);
}


/**
 * 删除产品
 * @param id
 */
async function productDelete(id: number): Promise<void> {
    console.log("product delete ", id)
    await del<void>(`/product/${id}`);
}

export {productPage, productEdit, productDelete, productAdd}

export type {ProductType}