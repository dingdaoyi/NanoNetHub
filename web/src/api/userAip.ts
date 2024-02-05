import {post} from "../config/http.ts";

type LoginType = {
    username?: string;
    password?: string;
    remember?: string;
};

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
export type {LoginType};
