<script lang="ts">
    import Button, { Label } from '@smui/button';
    import Dialog, { Actions, Content, Title } from '@smui/dialog';
    import { createEventDispatcher } from 'svelte';
    import type { Item } from '../types/models';

    export let open = false;
    export let title: string;
    export let description: string;
    export let confirmEventName: string;
    export let selectedItem: Item;

    const dispatch = createEventDispatcher();

    const handleCancel = () => {
        dispatch('cancel');
    };

    const handleConfirm = () => {
        dispatch(confirmEventName);
    };
</script>

<Dialog
    bind:open
    on:SMUIDialog:closed={handleCancel}
    aria-labelledby="edit-modal-title"
    aria-describedby="edit-modal-content"
>
    <Title id="edit-modal-title">{selectedItem._key}: {title}</Title>
    <Content id="edit-modal-content">{description}</Content>
    <Actions>
        <Button on:click={handleConfirm}>
            <Label>Confirm</Label>
        </Button>
        <Button on:click={handleCancel}>
            <Label>Cancel</Label>
        </Button>
    </Actions>
</Dialog>
