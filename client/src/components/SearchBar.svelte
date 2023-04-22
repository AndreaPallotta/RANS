<script lang="ts">
    import Button from '@smui/button/src/Button.svelte';
    import Checkbox from '@smui/checkbox';
    import FormField from '@smui/form-field';
    import IconButton from '@smui/icon-button';
    import { createEventDispatcher, onMount } from 'svelte';
    import authStore from '../store/auth.store';
    import { Role } from '../types/ifaces';

    let query = '';
    let timeoutId: ReturnType<typeof setTimeout> | undefined;
    export let checked = false;

    const dispatch = createEventDispatcher();

    const handleSearch = () => {
        if (!query || query.trim().length === 0) {
            dispatch('clear');
        } else {
            clearTimeout(timeoutId);
            timeoutId = setTimeout(handleDispatchSearch, 500);
        }
    };

    const handleDispatchSearch = () => {
        dispatch('search', query);
    };

    const handleClear = () => {
        query = '';
        dispatch('clear');
    };

    const handleAdd = () => {
        dispatch('add');
    };

    const handleCheckboxToggle = (e: any) => {
        dispatch('toggle', e.target.checked);
    };

    onMount(() => {
        const input = document.getElementById('search-input');
        input.focus();
    });
</script>

<div id="search-bar">
    <input
        id="search-input"
        type="text"
        placeholder="Search..."
        bind:value={query}
        on:input={handleSearch}
    />
    {#if query}
        <IconButton class="material-icons" on:click={handleClear}
            >clear</IconButton
        >
    {/if}
    {#if $authStore.role == Role.VENDOR}
        <FormField>
            <Checkbox bind:checked on:change={handleCheckboxToggle} touch />
            <span slot="label">Show owned items</span>
        </FormField>
        <Button
            on:click={handleAdd}
            variant="raised"
            style="padding: 1.5rem; flex: 0.1">Add Item</Button
        >
    {/if}
</div>
