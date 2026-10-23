import AppConsts from "./app-consts";
import axios, { AxiosInstance, InternalAxiosRequestConfig } from "axios";
import {message} from 'antd';
import { handleError } from "./error-handler";
import {storageUtils} from "@/utils/storageUtils.ts";

export interface IResponse {
  code: number | string;
  data: any;
  msg: string;
}

let _instance: AxiosInstance | null = null;

const TOKEN_EXPIRED_CODE = 3002;
let refreshing: Promise<string> | null = null;

// 同一时间只发一次刷新请求，并发的过期请求共用结果（RefreshToken 是一次性的）
function refreshAccessToken(): Promise<string> {
  if (!refreshing) {
    refreshing = axios
      .post(`${AppConsts.remoteServiceBaseUrl}/api/system/user/refreshToken`, {
        refreshToken: storageUtils.getRefreshToken(),
      })
      .then((r) => {
        const { token, expiresAt, refreshToken, refreshExpiresAt } = r.data.data;
        storageUtils.saveToken(token);
        storageUtils.saveExpiresAt(expiresAt);
        storageUtils.saveRefreshToken(refreshToken, refreshExpiresAt);
        return token as string;
      })
      .finally(() => {
        refreshing = null;
      });
  }
  return refreshing;
}

export function apiHttp(): AxiosInstance {
  if (!_instance) {
    _instance = axios.create({
      baseURL: AppConsts.remoteServiceBaseUrl,
    });
    _instance.defaults.withCredentials = true;

    _instance.interceptors.response.use(
      (r) => {
        // const d = r?.data as Record<string, unknown>;
        // if ( d && typeof d === "object" && d.__abp && "result" in d &&(d.success === undefined || d.success === true)) {
        //   r.data = d.result;
        // }
        return r;
      },
      async (error) => {
        const cfg = error?.config as (InternalAxiosRequestConfig & { _retried?: boolean }) | undefined;
        if (cfg && !cfg._retried && error.response?.data?.code === TOKEN_EXPIRED_CODE && storageUtils.getRefreshToken()) {
          cfg._retried = true;
          try {
            const token = await refreshAccessToken();
            cfg.headers["Authorization"] = `Bearer ${token}`;
            return _instance!(cfg);
          } catch {
            storageUtils.logout();
            window.location.href = `${import.meta.env.BASE_URL}login`;
            return Promise.reject(error);
          }
        }
        handleError(error);
        return Promise.reject(error);
      },
    );

    _instance.interceptors.request.use((cfg) => {
      try {
        cfg.headers = cfg.headers || ({} as Record<string, unknown>);
        const token = storageUtils.getToken();
        if (token && !cfg.headers["Authorization"]) {
          cfg.headers["Authorization"] = `Bearer ${token}`;
        }
        try {
          const m = document.cookie.match(
            /(?:^|; )Abp\.Localization\.CultureName=([^;]*)/,
          );
          const cookieLang = m ? decodeURIComponent(m[1]) : "";
          if (cookieLang && !cfg.headers[".AspNetCore.Culture"]) {
            cfg.headers[".AspNetCore.Culture"] =
              `c=${cookieLang}|uic=${cookieLang}`;
          }
        } catch {
          /* ignore */
        }
        try {
          const tenantCookieName = "X-TenantId";
          const getTenantId = () => {
            try {
              return undefined;
            } catch {
              return undefined;
            }
          };
          let tenantId: string | number | undefined = getTenantId();
          if (!tenantId) {
            const re = new RegExp(
              "(?:^|; )" +
                tenantCookieName.replace(/([.$?*|{}()[\\/+^])/g, "\\$1") +
                "=([^;]*)",
            );
            const m2 = document.cookie.match(re);
            tenantId = m2 ? decodeURIComponent(m2[1]) : "";
          }
          if (
            tenantId &&
            !(tenantCookieName in (cfg.headers as Record<string, unknown>))
          ) {
            (cfg.headers as Record<string, unknown>)[tenantCookieName] =
              tenantId + "";
          }
        } catch {
          message?.error?.("An error occurred, please try again later.");
        }
        if (!cfg.headers["Accept"]) cfg.headers["Accept"] = "application/json";
      } catch {
        message?.error?.("An error occurred, please try again later.");
      }
      return cfg;
    });
  }
  return _instance;
}

export default apiHttp;
