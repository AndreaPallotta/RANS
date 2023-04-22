import type Snackbar from '@smui/snackbar';
import { writable } from 'svelte/store';

export type NotificationVariant = 'success' | 'warning' | 'error' | undefined;

export interface INotification {
    snackbar: Snackbar;
    message: string;
    variant?: NotificationVariant;
    open: (message: string, variant?: NotificationVariant) => void;
}

const notifStore = writable<INotification>(null);

export default notifStore;
