import {post} from "../config/http.ts";
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

export {productPage}

export type {ProductType}