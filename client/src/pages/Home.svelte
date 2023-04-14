<script lang="ts">
  import LayoutGrid, { Cell } from "@smui/layout-grid"
  import { onMount } from "svelte"
  import type { Updater } from "svelte/store"
  import ConfirmationModal from "../components/ConfirmationModal.svelte"
  import ItemCard from "../components/ItemCard.svelte"
  import ItemModal from "../components/ItemModal.svelte"
  import OrderModal from "../components/OrderModal.svelte"
  import SearchBar from "../components/SearchBar.svelte"
  import authStore from "../store/auth.store"
  import itemsStore from "../store/items.store"
  import notifStore from "../store/notification.store"
  import ordersStore from "../store/orders.store"
  import type {
    AddOrderReq,
    DeleteItemReq,
    DeleteItemRes,
    UpdateItemReq,
  } from "../types/ifaces"
  import type { IOrder, Item } from "../types/models"
  import {
    axiosDelete,
    axiosGet,
    axiosPost,
    axiosPut,
  } from "../utils/api.utils"
  import { objectDifference } from "../utils/utils"

  let open = false
  let confirmOpen = false
  let orderOpen = false
  let selectedItem: Item = null

  const getItems = async () => {
    const response = await axiosGet<Item[], unknown>("/api/get_items")

    if (response.error || !response.data) {
      $notifStore.open(response.error ?? "Error retrieving items", "error")
      return
    }

    $itemsStore = response.data.content
  }

  const searchItem = async (e: CustomEvent<string>) => {
    const response = await axiosGet<Item[], unknown>(
      `/api/get_item/${e.detail}`
    )

    if (response.error || !response.data) {
      $notifStore.open(
        response.error ?? `Error searching for ${e.detail}`,
        "error"
      )
      return
    }

    $itemsStore = response.data.content
  }

  const deleteItem = async () => {
    const response = await axiosDelete<DeleteItemRes, DeleteItemReq>(
      "/api/delete_item",
      { id: selectedItem._key }
    )
    if (response.error || !response.data) {
      $notifStore.open(
        response.error ?? `Error deleting for ${selectedItem}`,
        "error"
      )
      return
    }

    await getItems()
    $notifStore.open(
      `Successfully deleted "${response.data.content.name}"`,
      "success"
    )
  }

  const saveEditedItem = async (e: CustomEvent<Item>) => {
    const body = objectDifference<UpdateItemReq, Item>(selectedItem, e.detail, {
      id: selectedItem._key,
    })

    const response = await axiosPut<Item, UpdateItemReq>("/api/edit_item", body)

    if (response.error || !response.data) {
      $notifStore.open(
        response.error ?? `Error updating ${e.detail.name}`,
        "error"
      )
      return
    }

    await getItems()
    $notifStore.open(`Successfully updated "${selectedItem.name}"`, "success")
    closeModal()
  }

  const addNewItem = async (e: CustomEvent<Partial<Item>>) => {
    const response = await axiosPost<Item, Partial<Item>>(
      "/api/add_item",
      e.detail
    )

    if (response.error || !response.data) {
      $notifStore.open(response.error ?? "Error creating new item", "error")
      return
    }

    await getItems()
    $notifStore.open(`Successfully created "${e.detail.name}"`, "success")
    closeModal()
  }

  const orderItem = async (e: CustomEvent<number>) => {
    const body: AddOrderReq = {
      user_id: $authStore._key,
      item_id: selectedItem._key,
      item_name: selectedItem.name,
      quantity: e.detail,
      quantity_diff: selectedItem.quantity - e.detail,
      price: e.detail * selectedItem.price,
    }

    const response = await axiosPost<IOrder, AddOrderReq>(
      "/api/add_order",
      body
    )

    if (!response.data && response.error) {
      $notifStore.open(response.error ?? "Error creating order", "error")
      return
    }

    ordersStore.update((prev: IOrder[]) => [...prev, response.data.content])

    const { quantity, price } = response.data.content

    await getItems()
    $notifStore.open(
      `Successfully ordered ${quantity} for $${price}`,
      "success"
    )
    closeOrderModal()
  }

  const openModal = (e: CustomEvent<Item>) => {
    open = true
    selectedItem = e.detail
  }

  const closeModal = () => {
    open = false
    selectedItem = null
  }

  const openConfirmModal = (e: CustomEvent<Item>) => {
    confirmOpen = true
    selectedItem = e.detail
  }

  const closeConfirmModal = () => {
    confirmOpen = false
    selectedItem = null
  }

  const openOrderModal = (e: CustomEvent<Item>) => {
    orderOpen = true
    selectedItem = e.detail
  }

  const closeOrderModal = () => {
    orderOpen = false
    selectedItem = null
  }

  onMount(() => {
    getItems()
    return () => ($itemsStore = [])
  })
</script>

<div class="page-container">
  <SearchBar on:search={searchItem} on:clear={getItems} on:add={openModal} />
  <LayoutGrid>
    {#each $itemsStore as item}
      <Cell>
        <ItemCard
          {item}
          on:delete={openConfirmModal}
          on:edit={openModal}
          on:order={openOrderModal}
        />
      </Cell>
    {/each}
  </LayoutGrid>

  {#if open}
    <ItemModal
      itemToEdit={selectedItem}
      {open}
      on:close={closeModal}
      on:update={saveEditedItem}
      on:add={addNewItem}
    />
  {/if}

  {#if selectedItem && confirmOpen}
    <ConfirmationModal
      open={confirmOpen}
      title="Confirm Delete"
      description={`Are you sure you want to delete '${selectedItem.name}'`}
      confirmEventName="confirm_delete"
      {selectedItem}
      on:cancel={closeConfirmModal}
      on:confirm_delete={deleteItem}
    />
  {/if}

  {#if selectedItem && orderOpen}
    <OrderModal
      open={orderOpen}
      item={selectedItem}
      on:cancel={closeOrderModal}
      on:confirm={orderItem}
    />
  {/if}
</div>
