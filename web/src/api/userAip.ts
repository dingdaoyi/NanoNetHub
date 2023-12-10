import {post} from "../config/http.ts";
import {UserInfo} from "../store/userInfo.tsx";

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
    return await post<UserInfo>('/login', params);
}

export {
    loginApi
};
export type {LoginType};
