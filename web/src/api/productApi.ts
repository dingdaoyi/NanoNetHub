import {del, get, post, put} from "../utils/http.ts";
import {PageResult} from "../types/type_def.ts";
import {ProductDict, ProductType} from "../types/api.ts";

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
