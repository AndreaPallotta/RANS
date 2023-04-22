<script lang="ts">
    import Button, { Label } from '@smui/button';
    import LayoutGrid, { Cell } from '@smui/layout-grid';
    import { onMount } from 'svelte';
    import OrderCard from '../components/OrderCard.svelte';
    import authStore from '../store/auth.store';
    import notifStore from '../store/notification.store';
    import ordersStore from '../store/orders.store';
    import type { DeleteOrderReq } from '../types/ifaces';
    import type { IOrder } from '../types/models';
    import { axiosDelete, axiosGet } from '../utils/api.utils';

    $: isClearDisabled = $ordersStore.length < 1 || !$authStore;

    const getOrders = async () => {
        const response = await axiosGet<IOrder[], unknown>(
            `/api/get_orders/${$authStore._key}`
        );

        if (response.error || !response.data) {
            $notifStore.open(
                response.error ?? 'Error retrieving orders',
                'error'
            );
            return;
        }

        ordersStore.set(response.data.content);
    };

    const clearOrders = async () => {
        const response = await axiosDelete<IOrder[], DeleteOrderReq>(
            '/api/delete_orders',
            { user_id: $authStore._key }
        );

        if (response.error || !response.data) {
            $notifStore.open(
                response.error ?? 'Error deleting orders',
                'error'
            );
            return;
        }

        ordersStore.set(response.data.content);
    };

    onMount(() => {
        getOrders();
        return () => ordersStore.set([]);
    });
</script>

<div style="padding-top: 5rem; margin-left: 3rem; margin-right: 3rem;">
    <div style="display: flex; flex-direction: row;">
        <h2 style="margin-left: 1rem; margin-right: 1rem;">Your Orders</h2>
        <Button
            disabled={isClearDisabled}
            color="primary"
            class="button"
            variant="raised"
            style="margin-top: 24px"
            type="submit"
            on:click={clearOrders}
        >
            <Label>Clear Orders</Label>
        </Button>
    </div>
    <LayoutGrid>
        {#each $ordersStore as order}
            <Cell span={3}>
                <OrderCard {order} />
            </Cell>
        {/each}
    </LayoutGrid>
</div>
