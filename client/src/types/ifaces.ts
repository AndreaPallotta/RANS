import type { IUser, Item } from "./models";

export interface AuthRes {
    user: IUser;
    token: string;
}

export interface DeleteItemReq {
    id: string;
}

export interface DeleteItemRes {
    name: string;
};

export type UpdateItemReq = {
    id: string;
} & Partial<Item>;