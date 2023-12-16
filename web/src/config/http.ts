/**
 * 网络请求配置
 */
import axios from "axios";

axios.defaults.timeout = 100000;
axios.defaults.baseURL = "/api";

interface R<T> {
    code: number,
    success: boolean,
    msg?: string,
    data?: T,
}


/**
 * http response 拦截器
 */
axios.interceptors.response.use(
    (response) => {
        if (response.data?.success === false) {
            console.log("响应失败:", response.data);
        }
        return response.data;
    },
    (error) => {
        console.log("请求出错：", error);
    }
);

/**
 * @param url  请求url
 * @param params  请求参数
 * @returns {Promise}
 */
export function get<T>(url: string, params?: object): Promise<T> {
    return new Promise((resolve, reject) => {
        axios.get(url, {
            params: params,
        }).then((response) => {
            resolve(response.data);
        })
            .catch((error) => {
                reject(error);
            });
    });
}


/**
 * @param url  请求url
 * @param params  请求参数
 * @returns {Promise}
 */
export function del<T>(url: string, params?: object): Promise<T> {
    return new Promise((resolve, reject) => {
        axios.delete(url, {
            params: params,
        }).then((response) => {
            resolve(response.data);
        })
            .catch((error) => {
                reject(error);
            });
    });
}

/**
 * @param url
 * @param data
 * @returns {Promise}
 */

export function post<T>(url: string, data: object): Promise<T> {
    return new Promise((resolve, reject) => {
        axios.post(url, data).then(
            (response) => {
                resolve(response.data);
            },
            (err) => {
                reject(err);
            }
        );
    });
}

/**
 * @param url
 * @param data
 * @returns {Promise}
 */
export function put<T>(url: string, data: object): Promise<T> {
    return new Promise((resolve, reject) => {
        axios.put(url, data).then(
            (response) => {
                resolve(response.data);
            },
            (err) => {
                msag(err.response);
                reject(err);
            }
        );
    });
}

//失败提示
function msag(err: { response: { status: number; data: R<object>; }; }) {
    if (err && err.response) {
        switch (err.response.status) {
            case 400:
                alert(err.response.data.msg);
                break;
            case 401:
                alert("未授权，请登录");
                break;

            case 403:
                alert("拒绝访问");
                break;

            case 404:
                alert("请求地址出错");
                break;

            case 408:
                alert("请求超时");
                break;

            case 500:
                alert("服务器内部错误");
                break;

            case 501:
                alert("服务未实现");
                break;

            case 502:
                alert("网关错误");
                break;

            case 503:
                alert("服务不可用");
                break;

            case 504:
                alert("网关超时");
                break;

            case 505:
                alert("HTTP版本不受支持");
                break;
            default:
        }
    }
}


