import { writable } from 'svelte/store';
import type { Role } from '../types/ifaces';
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
    role?: Role;
}

const authStore = writable<IUser>(null);
export const jwtStore = writable<string>(null);

export default authStore;
