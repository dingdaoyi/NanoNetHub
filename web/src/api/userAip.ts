import {post} from "@/utils/http.ts";
import {LoginType} from "@/types/api.ts";


/**
 * 登录
 * @param params
 */
async function loginApi(params: LoginType) {
    return await post<string>('/login', params);
}

export {
    loginApi
};
