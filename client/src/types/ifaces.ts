import type { IUser, Item } from './models';

export interface AuthRes {
    user: IUser;
    token: string;
};

export interface DeleteItemReq {
    id: string;
};

export interface DeleteItemRes {
    name: string;
};

export type AddItemReq = {
    user_id: string;
} & Partial<Item>;

export type UpdateItemReq = {
    id: string;
} & Partial<Item>;

export enum Role {
    CUSTOMER = 'CUSTOMER',
    VENDOR = 'VENDOR',
};

export type AddOrderReq = {
    user_id: string;
    item_id: string;
    item_name: string;
    quantity: number;
    quantity_diff: number;
    price: number;
};

export type DeleteOrderReq = {
    user_id: string;
};
