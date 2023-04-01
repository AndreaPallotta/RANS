<script lang="ts">
  import Button from "@smui/button/src/Button.svelte";
  import IconButton from '@smui/icon-button';
  import { createEventDispatcher, onMount } from "svelte";

  let query = "";

  const dispatch = createEventDispatcher();

  const handleSearch = () => {
    dispatch("search", query);
  };

  const handleClear = () => {
    query = "";
    dispatch("clear");
  };

  const handleAdd = () => {
    dispatch("add");
  };

  onMount(() => {
    const input = document.getElementById("search-input");
    input.focus();
  });
</script>

<div id="search-bar">
  <input
    id="search-input"
    type="text"
    placeholder="Search..."
    bind:value={query}
  />
  {#if query}
    <IconButton class="material-icons" on:click={handleClear}>clear</IconButton>
  {/if}
  <Button on:click={query.trim().length === 0 ? handleClear : handleSearch} variant="raised" style="padding: 1.5rem; flex: 0.1">Search</Button>
  <Button on:click={handleAdd} variant="raised" style="padding: 1.5rem; flex: 0.1">Add Item</Button>
</div>