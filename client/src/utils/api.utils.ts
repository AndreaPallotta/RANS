import axios, { type InternalAxiosRequestConfig } from 'axios';
import { get } from 'svelte/store';
import authStore, { jwtStore } from '../store/auth.store';
import type { AuthRes } from '../types/ifaces';
import { clearState, setState } from './utils';

interface ApiResponse<T> {
    result: string;
    content: T;
}

interface AxiosResponse<T> {
    status?: number;
    data?: ApiResponse<T>;
    error?: string;
}

const baseURL = 'http://localhost:3000';

export const client = axios.create({
    baseURL,
    headers: {
        'Content-Type': 'application/json',
    },
});

client.interceptors.request.use((config: InternalAxiosRequestConfig) => {
    const token = get(jwtStore);
    if (token) {
        config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
}, (err: any) => Promise.reject(err));

client.interceptors.response.use(
    (res: any) => res,
    async (err: any) => {
        if (err.response && err.response.status === 401 && err.config && !err.config.__isRetry) {
            await refresh();

            const originalRequest = err.config;
            originalRequest.__isRetry = true;
            return client(originalRequest);
        }
        return Promise.reject(err);
    }
);

export async function axiosGet<T, B>(endpoint: string, params?: B): Promise<AxiosResponse<T>> {
    try {
        const { status, data } = await client.get(endpoint, { data: params });
        return { status, data };
    } catch (err) {
        if (axios.isAxiosError(err)) {
            const { data, status } = err.response ?? {};
            return { error: data?.content?.error_msg || err.message, status };
        }
        return { error: `Request failed with unknown error: ${err.message}` };
    }
};

export async function axiosPost<T, B>(endpoint: string, body: B): Promise<AxiosResponse<T>> {
    try {
        const { status, data } = await client.post(endpoint, body);
        return { status, data };
    } catch (err) {
        if (axios.isAxiosError(err)) {
            const { data, status } = err.response ?? {};
            return { error: data?.content?.error_msg || err.message, status };
        }
        return { error: `Request failed with unknown error: ${err.message}` };
    }
};

export async function axiosPut<T, B>(endpoint: string, body: B): Promise<AxiosResponse<T>> {
    try {
        const { status, data } = await client.put(endpoint, body);
        return { status, data };
    } catch (err) {
        if (axios.isAxiosError(err)) {
            const { data, status } = err.response ?? {};
            return { error: data?.content?.error_msg || err.message, status };
        }
        return { error: `Request failed with unknown error: ${err.message}` };
    }
};

export async function axiosDelete<T, B>(endpoint: string, body: B): Promise<AxiosResponse<T>> {
    try {
        const { status, data } = await client.delete(endpoint, { data: body });
        return { status, data };
    } catch (err) {
        if (axios.isAxiosError(err)) {
            const { data, status } = err.response ?? {};
            return { error: data?.content?.error_msg || err.message, status };
        }
        return { error: `Request failed with unknown error: ${err.message}` };
    }
};

const refresh = async () => {
    const email = get(authStore).email;

    if (!email) {
        clearState();
        return;
    }

    const response = await axiosGet<AuthRes, unknown>(`/api/auth/refresh/${email}`);

    console.log(response);

    if (!response.data.content || response.error) {
        clearState();
    }

    const { user, token } = response.data.content;

    try {
        setState(user, token);
    } catch { }
}