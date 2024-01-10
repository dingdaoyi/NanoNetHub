import {del, get, post, put} from "../config/http.ts";
import {PageResult} from "../common/type_def.ts";

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


/**
 * 分页查询产品
 * @param params
 */
async function productPage(params: {
    product_name?: string,
    base_query: { page: number, size: number }
}): Promise<PageResult<ProductType>> {
    return await post<PageResult<ProductType>>('/product/page', params);
}

/**
 * 分页查询产品
 * @param params
 */
async function productDict(): Promise<ProductDict[]> {
    return await get<ProductDict[]>('/product/dict');
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
 * 修改产品
 * @param id
 */
async function productDetails(id: number): Promise<ProductType> {
    return await get<ProductType>(`/product/${id}`);
}

/**
 * 添加产品
 * @param params
 */
async function productAdd(params: ProductType): Promise<void> {
    await post<void>('/product', params);
}


/**
 * 删除产品
 * @param id
 */
async function productDelete(id: number): Promise<void> {
    await del<void>(`/product/${id}`);
}

export {productPage, productEdit, productDelete, productAdd, productDetails, productDict}

export type {ProductType, ProductDict}