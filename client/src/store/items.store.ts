import { writable } from 'svelte/store';
import type { Item } from '../types/models';

const itemsStore = writable<Item[]>([]);

export default itemsStore;