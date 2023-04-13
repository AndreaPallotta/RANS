<div style="padding-top: 5rem; margin-left: 3rem; margin-right: 3rem;">
    <h2 style="padding-left: 1rem;">Your Orders</h2>
    <LayoutGrid>
        {#each $ordersStore as order}
            <Cell span={3}>
                <OrderCard {order} />
            </Cell>
        {/each}
    </LayoutGrid>
</div>

<script lang="ts">
    import LayoutGrid, { Cell } from '@smui/layout-grid';
    import { onMount } from "svelte";
    import OrderCard from '../components/OrderCard.svelte';
    import authStore from "../store/auth.store";
    import notifStore from "../store/notification.store";
    import ordersStore from "../store/orders.store";
    import type { IOrder } from "../types/models";
    import { axiosGet } from "../utils/api.utils";

    const getOrders = async () => {
        const response = await axiosGet<IOrder[], unknown>(`/api/get_orders/${$authStore._key}`);

        if (response.error || !response.data) {
            $notifStore.open(response.error ?? 'Error retrieving orders', 'error');
            return;
        }

        ordersStore.set(response.data.content);
    };

    onMount(() => {
    getOrders();
    return () => $ordersStore = [];
  });
</script>