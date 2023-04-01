<script lang="ts">
  import LayoutGrid, { Cell } from '@smui/layout-grid';
  import { onMount } from 'svelte';
  import ItemCard from "../components/ItemCard.svelte";
  import ItemModal from '../components/ItemModal.svelte';
  import SearchBar from '../components/SearchBar.svelte';
  import itemsStore from '../store/items.store';
  import notifStore from '../store/notification.store';
  import type { DeleteItemReq, DeleteItemRes, UpdateItemReq } from '../types/ifaces';
  import type { Item } from '../types/models';
  import { axiosDelete, axiosGet, axiosPost, axiosPut } from '../utils/api.utils';
  import { objectDifference } from '../utils/utils';

  let open = false;
  let itemToEdit = null;

  const getItems = async () => {
    const response = await axiosGet<Item[], unknown>('/api/get_items');

    if (response.error || !response.data) {
        $notifStore.open(response.error ?? 'Error retrieving items', 'error');
        return;
    }

    $itemsStore = response.data.content;
  };

  const searchItem = async (e: CustomEvent<string>) => {
    const response = await axiosGet<Item[], unknown>(`/api/get_item/${e.detail}`);

    if (response.error || !response.data) {
        $notifStore.open(response.error ?? `Error searching for ${e.detail}`, 'error');
        return;
    }

    $itemsStore = response.data.content;
  };

  const deleteItem = async (e: CustomEvent<string>) => {
    const response = await axiosDelete<DeleteItemRes, DeleteItemReq>('/api/delete_item', { id: e.detail });

    if (response.error || !response.data) {
        $notifStore.open(response.error ?? `Error deleting for ${e.detail}`, 'error');
        return;
    }

    await getItems();
    $notifStore.open(`Successfully deleted "${response.data.content.name}"`, 'success');
  };

  const saveEditedItem = async (e: CustomEvent<Item>) => {
    const body = objectDifference<UpdateItemReq, Item>(itemToEdit, e.detail, { id: itemToEdit._key });

    const response = await axiosPut<Item, UpdateItemReq>('/api/edit_item', body);

    if (response.error || !response.data) {
        $notifStore.open(response.error ?? `Error updating ${e.detail.name}`, 'error');
        return;
    }

    await getItems();
    $notifStore.open(`Successfully updated "${itemToEdit.name}"`, 'success');
    closeModal();
  };

  const addNewItem = async (e: CustomEvent<Partial<Item>>) => {

    const response = await axiosPost<Item, Partial<Item>>('/api/add_item', e.detail);

    if (response.error || !response.data) {
      $notifStore.open(response.error ?? 'Error creating new item', 'error');
      return;
    }

    await getItems();
    $notifStore.open(`Successfully created "${e.detail.name}"`, 'success');
    closeModal();
  };

  const openModal = (e: CustomEvent<Item>) => {
    open = true;
    itemToEdit = e.detail;
  };

  const closeModal = () => {
    open = false;
    itemToEdit = null;
  };

  onMount(() => {
    getItems();
    return () => $itemsStore = [];
  });
</script>

<div id="home-wrapper">
    <SearchBar on:search={searchItem} on:clear={getItems} on:add={openModal} />
    <LayoutGrid>
    {#each $itemsStore as item}
        <Cell>
            <ItemCard {item} on:delete={deleteItem} on:edit={openModal} />
        </Cell>
    {/each}
    </LayoutGrid>

    <ItemModal {itemToEdit} {open} on:close={closeModal} on:update={saveEditedItem} on:add={addNewItem} />
</div>