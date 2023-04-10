import authStore, { jwtStore } from "../store/auth.store";
import type { IUser } from "../types/models";

export function objectDifference<T, U extends Record<string, any>>(first: U, second: U, defaultValues: Partial<T> = {}): T {
    return Object.entries(second).reduce((acc, [k, v]) => {
        if (first[k] !== v) {
            acc[k] = v;
        }
        return acc;
    }, defaultValues as T);
};

type NumberLiteral = {
    [key: string]: number;
};

const numberLiteralMap: NumberLiteral = {
    B: 1000000000,
    M: 1000000,
    K: 1000,
};

export const formatNumberLiteral = (value: number) => {
    const formatter = new Intl.NumberFormat('en-US', { notation: 'compact' });

    for (const letter in numberLiteralMap) {
        const letterValue = numberLiteralMap[letter];
        if (value >= letterValue) {
            return `${formatter.format(value / letterValue)}${letter}`;
        }
    }

    return value;
};

export const setState = (user: IUser, token: string) => {
    jwtStore.set(token);
    authStore.set(user);

    localStorage.setItem('jwt', token);
    localStorage.setItem('user', JSON.stringify(user));
}

export const clearState = (clearStorage: boolean = true, redirect: boolean = true, path: string = '/login') => {
    jwtStore.set(null);
    authStore.set(null);

    if (clearStorage) {
        localStorage.removeItem('jwt');
        localStorage.removeItem('user');
    }

    if (redirect) window.location.replace(path);
};

