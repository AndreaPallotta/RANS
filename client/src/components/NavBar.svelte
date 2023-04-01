<script lang="ts">
  import Button from "@smui/button/src/Button.svelte";
  import IconButton from "@smui/icon-button";
  import TopAppBar, {
    Row,
    Section,
    Title
  } from '@smui/top-app-bar';
  import { onMount } from "svelte";
  import { Link } from 'svelte-navigator';
  import authStore from "../store/auth.store";
  import type { IUser } from "../types/models";

  export let topAppBar: TopAppBar;

  onMount(() => {
    $authStore = JSON.parse(localStorage.getItem("auth")) as IUser;
  });

  const handleLogout = () => {
    $authStore = null;
    localStorage.removeItem("auth");
  };
</script>

<TopAppBar bind:this={topAppBar} variant="standard" color="primary">
  <Row>
    <Section>
        <IconButton class="material-icons">menu</IconButton>
        <Title>E-Commerce With RANS</Title>
    </Section>
    <Section align="end" toolbar>
        {#if $authStore === null}
            <Link to="/login" class="nav-link">Login</Link>
            <Link to="/signup" class="nav-link">Sign up</Link>
        {:else}
            <Link to="/" class="nav-link">Home</Link>
            <Link to="/login" class="nav-link" on:click={handleLogout}>Log Out</Link>
        {/if}
    </Section>
  </Row>
</TopAppBar>