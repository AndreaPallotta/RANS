import type { Item } from "./models";

export interface DeleteItemReq {
    id: string;
}

export interface DeleteItemRes {
    name: string;
};

export type UpdateItemReq = {
    id: string;
} & Partial<Item>;