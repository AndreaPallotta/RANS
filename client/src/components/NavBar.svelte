<script lang="ts">
    import IconButton from '@smui/icon-button';
    import TopAppBar, { Row, Section, Title } from '@smui/top-app-bar';
    import { Link } from 'svelte-navigator';
    import authStore from '../store/auth.store';
    import { Role } from '../types/ifaces';

    export let topAppBar: TopAppBar;

    const handleLogout = () => {
        authStore.set(null);
        localStorage.removeItem('user');
    };
</script>

<TopAppBar bind:this={topAppBar} variant="standard" color="primary">
    <Row>
        <Section>
            <IconButton class="material-icons">menu</IconButton>
            <Link to="/" class="nav-link">
                <Title>E-Commerce With RANS</Title>
            </Link>
        </Section>
        <Section align="end" toolbar>
            {#if $authStore === null}
                <Link to="/login" class="nav-link">Login</Link>
                <Link to="/signup" class="nav-link">Sign up</Link>
            {:else}
                <Link to="/" class="nav-link">Home</Link>
                {#if $authStore.role == Role.CUSTOMER}
                    <Link to="/orders" class="nav-link">Orders</Link>
                {/if}
                <Link to="/login" class="nav-link" on:click={handleLogout}
                    >Log Out</Link
                >
            {/if}
        </Section>
    </Row>
</TopAppBar>
