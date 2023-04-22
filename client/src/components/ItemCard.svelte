<script lang="ts">
    import Button, { Label } from '@smui/button';
    import Card, {
        ActionButtons,
        Actions,
        Content,
        Media,
        PrimaryAction,
    } from '@smui/card';
    import { createEventDispatcher } from 'svelte';
    import authStore from '../store/auth.store';
    import { Role } from '../types/ifaces';
    import type { Item } from '../types/models';
    import { formatNumberLiteral } from '../utils/utils';

    const dispatch = createEventDispatcher();

    const handleDelete = () => {
        dispatch('delete', item);
    };

    const handleEdit = () => {
        dispatch('edit', item);
    };

    const handleOrder = () => {
        dispatch('order', item);
    };

    export let item: Item;
</script>

<div class="card-display">
    <div class="card-container">
        <Card>
            <PrimaryAction>
                <Media class="card-media-16x9" aspectRatio="16x9" />
                <Content class="mdc-typography--body2">
                    <h2 class="mdc-typography--headline6" style="margin: 0;">
                        {item.name}
                        {#if item.quantity === 0}
                            <span style="color: red;">dsa</span>
                        {/if}
                    </h2>
                    <div class="mdc-typography--headline6">
                        <b>Vendor ID:</b>
                        {item.user_id}
                    </div>
                    <h4
                        class="mdc-typography--headline4"
                        id="description-label"
                    >
                        {#if item.description.length === 0}
                            &nbsp;
                        {:else}
                            {item.description}
                        {/if}
                    </h4>

                    <div class="mdc-typography--headline6">
                        <b>Price:</b> ${item.price}
                    </div>
                    <div class="mdc-typography--headline6">
                        <b>Quantity:</b>
                        {formatNumberLiteral(item.quantity)}
                    </div>
                </Content>
            </PrimaryAction>
            <Actions>
                <ActionButtons>
                    {#if $authStore.role === Role.VENDOR && item.user_id === $authStore._key}
                        <Button on:click={handleEdit}>
                            <Label>Edit</Label>
                        </Button>
                        <Button on:click={handleDelete}>
                            <Label>Delete</Label>
                        </Button>
                    {:else if $authStore.role === Role.CUSTOMER}
                        <Button on:click={handleOrder}>
                            <Label>Order</Label>
                        </Button>
                    {/if}
                </ActionButtons>
            </Actions>
        </Card>
    </div>
</div>

<style>
    * :global(.card-media-16x9) {
        background-image: url(https://place-hold.it/320x180?text=16x9&fontsize=23);
    }
</style>
