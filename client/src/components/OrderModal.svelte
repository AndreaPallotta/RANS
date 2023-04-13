<Dialog
  bind:open
  on:SMUIDialog:closed={handleCancel}
  aria-labelledby="order-modal-title"
  aria-describedby="order-modal-content"
>
  <Title id="order-modal-title">Order {item.name}</Title>
  <Content id="order-modal-content">
    <Textfield
        variant="filled"
        bind:value={quantity}
        label="Quantity"
        type="number"
        inputmode="numeric"
        style="width: 100%; margin-bottom: 1rem;"
        required
        invalid={!isQuantityValid}
        on:input={handleInput}
    >
        <HelperText slot="helper">Note: Orders must be greater than 0 and less than the total amount</HelperText>
    </Textfield>
    <pre>Remaining quantity: {isNaN(item.quantity - quantity) ? item.quantity : item.quantity - quantity}</pre>
    <pre>Total cost: ${(isNaN(quantity * item.price) ? 0.00 : quantity * item.price).toFixed(2)}</pre>
  </Content>
  <Actions>
    <Button on:click={handleConfirm} disabled={!isQuantityValid}>
      <Label>Confirm</Label>
    </Button>
    <Button on:click={handleCancel}>
      <Label>Cancel</Label>
    </Button>
  </Actions>
</Dialog>

<script lang="ts">
  import Button, { Label } from '@smui/button';
  import Dialog, { Actions, Content, Title } from '@smui/dialog';
  import Textfield from '@smui/textfield';
  import HelperText from '@smui/textfield/helper-text';
  import { createEventDispatcher } from 'svelte';
  import type { Item } from '../types/models';

  export let open = false;
  export let item: Item;

  let quantity: number = 0;

  $: isQuantityValid = quantity >= 0 && quantity <= item.quantity;

  const dispatch = createEventDispatcher();

  function handleInput(event) {
    const value = parseInt(event.target.value, 10);

    if (value > item.quantity) {
      quantity = item.quantity;
    } else if (value < 0) {
      quantity = 0;
    } else {
      quantity = value;
    }
  }

  const handleCancel = () => {
    dispatch("cancel");
  };

  const handleConfirm = () => {
    dispatch("confirm", quantity);
  }
</script>