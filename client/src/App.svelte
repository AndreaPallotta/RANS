<script lang="ts">
  import TopAppBar, { AutoAdjust } from "@smui/top-app-bar";
  import { onMount } from "svelte";
  import { Route, Router } from "svelte-navigator";
  import NavBar from "./components/NavBar.svelte";
  import Notification from "./components/Notification.svelte";
  import ProtectedRoute from "./components/ProtectedRoute.svelte";
  import Home from "./pages/Home.svelte";
  import Login from "./pages/Login.svelte";
  import Orders from "./pages/Orders.svelte";
  import Signup from "./pages/Signup.svelte";
  import authStore, { jwtStore } from "./store/auth.store";
  import type { IUser } from "./types/models";

  let topAppBar: TopAppBar

  onMount(() => {
    authStore.set(JSON.parse(localStorage.getItem("user")) as IUser);
    jwtStore.set(localStorage.getItem("jwt"));
  })
</script>

<Router>
  <NavBar {topAppBar} />
  <AutoAdjust {topAppBar} class="page-wrapper">
    <ProtectedRoute path="/" let:location>
      <Home />
    </ProtectedRoute>
    <Route path="/login">
      <Login />
    </Route>
    <Route path="/signup">
      <Signup />
    </Route>
    <Route path="/orders">
      <Orders />
    </Route>
    <Route path="*">
      <div class="container">
        <p>404</p>
      </div>
    </Route>
  </AutoAdjust>
</Router>
<Notification />

<style>
  .container {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100vh;
  }
</style>
