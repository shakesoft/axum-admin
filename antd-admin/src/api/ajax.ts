import axios, {AxiosInstance, AxiosResponse, InternalAxiosRequestConfig} from 'axios';
import {message} from 'antd';
import {storageUtils} from "../utils/storageUtils";

export const showMessage = (status: number | string): string => {
    let message: string = "";
    switch (status) {
        case 400:
            message = "请求错误(400)";
            break;
        case 401:
            message = "未授权，请重新登录(401)";
            break;
        case 403:
            message = "拒绝访问(403)";
            break;
        case 404:
            message = "请求出错(404)";
            break;
        case 408:
            message = "请求超时(408)";
            break;
        case 500:
            message = "服务器错误(500)";
            break;
        case 501:
            message = "服务未实现(501)";
            break;
        case 502:
            message = "网络错误(502)";
            break;
        case 503:
            message = "服务不可用(503)";
            break;
        case 504:
            message = "网络超时(504)";
            break;
        case 505:
            message = "HTTP版本不受支持(505)";
            break;
        default:
            message = `连接出错(${status})!`;
    }
    return `${message}，请检查网络或联系管理员！`;
};


// 返回res.data的interface
export interface IResponse {
    code: number | string;
    data: any;
    msg: string;
    total: number
}

const baseUrl = import.meta.env.VITE_APP_PROXY_URL;
export const axiosInstance: AxiosInstance = axios.create({
    baseURL: baseUrl,
    headers: {
        Accept: "application/json",
        "Content-Type": "application/json"
    },
});

// axios实例拦截响应
axiosInstance.interceptors.response.use(
    (response: AxiosResponse) => {
        if (response.status === 200) {
            let {data} = response.data;
            if (Object.prototype.toString.call(data) === '[object Object]' && data.list) {
                response.data.total = data.total;
                response.data.data = data.list;
            }
            return response;
        } else {
            showMessage(response.status);
            return response;
        }
    },
    // 请求失败
    (error: any) => {
        const {response} = error;
        if (response) {
            // 请求已发出，但是不在2xx的范围
            if (response.data.msg) {
                message.error(response.data.msg);
            } else if (response.data) {
                message.error(response.data);
            } else {
                message.error(showMessage(response.status));
            }
            return Promise.reject(response.data);
        } else {
            message.error('网络连接异常,请稍后再试!');
            return Promise.reject('网络连接异常,请稍后再试!');
        }
    }
);

// axios实例拦截请求
axiosInstance.interceptors.request.use(
    (config: InternalAxiosRequestConfig) => {
        const token = storageUtils.getToken()
        if (token) {
            config.headers.Authorization = `Bearer ${token}`
        }

        return config;
    },
    (error: any) => {
        return Promise.reject(error);
    }
)



