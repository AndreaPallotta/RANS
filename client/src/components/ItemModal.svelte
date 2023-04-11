<Dialog
  bind:open
  on:SMUIDialog:closed={handleClose}
  aria-labelledby="edit-modal-title"
  aria-describedby="edit-modal-content"
  surface$style="width: 850px; max-width: calc(100vw - 32px);"
>
  <Title id="edit-modal-title">{title}</Title>
  <Content id="edit-modal-content">
        <Textfield
            variant="filled"
            bind:value={item.name}
            label="Name"
            style="width: 100%; margin-bottom: 1rem;"
            required
            invalid={!isNameValid}
        />

        <Textfield
            variant="filled"
            bind:value={item.description}
            textarea
            label="Description"
            style="width: 100%; margin-bottom: 1rem;"
        />

        <Textfield
            variant="filled"
            bind:value={item.price}
            label="Price"
            type="number"
            prefix="$"
            style="width: 100%; margin-bottom: 1rem;"
            required
            invalid={!isPriceValid}
        />

        <Textfield
            variant="filled"
            bind:value={item.quantity}
            label="Quantity"
            type="number"
            style="width: 100%; margin-bottom: 1rem;"
            required
            invalid={!isQuantityValid}
        />
  </Content>
  <Actions>
    <Button disabled={isButtonDisabled} on:click={handleSave}>
      <Label>Save</Label>
    </Button>
    <Button on:click={handleClose}>
      <Label>Close</Label>
    </Button>
  </Actions>
</Dialog>

<script lang="ts">
  import Button, { Label } from '@smui/button';
  import Dialog, { Actions, Content, Title } from '@smui/dialog';
  import Textfield from '@smui/textfield';
  import { createEventDispatcher } from 'svelte';
  import type { Item } from '../types/models';

  export let open = false;
  export let itemToEdit: Item;

  let isFormModified = false;
  let item: Item | Partial<Item> = {
    name: "",
    description: "",
    price: 0,
    quantity: 0,
  };

  if (itemToEdit) {
    item = {...itemToEdit} as Item;
  }

  $: {
    if (itemToEdit) {
      isFormModified = Object.keys(item).some(key => item[key] !== itemToEdit[key]);
    } else {
      isFormModified = true;
    }
  }
  $: isNameValid = item.name.trim().length > 0;
  $: isPriceValid = item.price > 0 && item.price <= 1000000;
  $: isQuantityValid = item.quantity >= 0 && item.price <= 1000000;
  $: isButtonDisabled = !isNameValid || !isPriceValid || !isQuantityValid || !isFormModified;

  const title = itemToEdit !== null ? `Edit "${item.name}"` : "Add New Item";

  const dispatch = createEventDispatcher();

  const handleClose = () => {
    dispatch("close");
  };

  const handleSave = () => {
    dispatch(itemToEdit === null ? "add" : "update", item);
  };
</script>