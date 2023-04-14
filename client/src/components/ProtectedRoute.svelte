<script lang="ts">
  import { Route } from "svelte-navigator"
  import ProtectedRouteGuard from "./ProtectedRouteGuard.svelte"

  export let path: string
</script>

<Route {path} let:params let:location let:navigate>
  <ProtectedRouteGuard>
    <slot {params} {location} {navigate} />
  </ProtectedRouteGuard>
</Route>
