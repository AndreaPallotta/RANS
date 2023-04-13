import { writable } from 'svelte/store';
import type { IOrder } from '../types/models';

const ordersStore = writable<IOrder[]>([]);

export default ordersStore;