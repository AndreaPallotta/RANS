import type { Role } from "./ifaces";

export interface Item {
    _key: string;
    _rev: string;
    _id: string;
    name: string;
    description: string;
    quantity: number;
    price: number;
}

export interface IUser {
    _key: string;
    _rev: string;
    _id: string;
    first_name: string;
    last_name: string;
    email: string;
    password: string;
    role: Role;
}

export interface IOrder {
    _key: string;
    _rev: string;
    _id: string;
    user_id: string;
    item_id: string;
    item_name: string;
    quantity: number;
    price: number;
    date: Date;
}