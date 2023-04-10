import { writable } from 'svelte/store';
import type { IUser } from '../types/models';

export interface ISignIn {
    email: string;
    password: string;
}

export interface ISignUp {
    first_name: string;
    last_name: string;
    email: string;
    password: string;
}

const authStore = writable<IUser>(null);
export const jwtStore = writable<string>(null);

export default authStore;