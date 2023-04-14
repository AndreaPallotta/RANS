<script lang="ts">
  import IconButton from "@smui/icon-button"
  import Snackbar, { Actions, Label } from "@smui/snackbar"
  import notifStore, {
    type NotificationVariant,
  } from "../store/notification.store"

  let snackbar: Snackbar

  const openNotification = (message: string, variant: NotificationVariant) => {
    $notifStore.message = message
    $notifStore.variant = variant

    if (snackbar) {
      snackbar.forceOpen()
    }
  }

  $notifStore = {
    snackbar,
    message: "",
    open: openNotification,
  }
</script>

<Snackbar bind:this={snackbar} class={$notifStore.variant}>
  <Label>{$notifStore.message}</Label>
  <Actions>
    <IconButton class="material-icons" title="Dismiss">close</IconButton>
  </Actions>
</Snackbar>
