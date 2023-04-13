<div class="container">
    <div style="background-color: rgb(207, 205, 204, 0.4); padding: 3rem; border-radius: 10px">
        <Textfield
            variant="filled"
            bind:value={signIn.email}
            label="Email"
            style="width: 100%; margin-bottom: 1rem;"
            required
            invalid={!isEmailValid}
        >
            <Icon class="material-icons" slot="leadingIcon">email</Icon>
        </Textfield>

        <Textfield
            variant="filled"
            bind:value={signIn.password}
            label="Password"
            {type}
            style="width: 100%; margin-bottom: 1rem;"
            required
            invalid={!isPasswordValid}
        >
            <Icon class="material-icons" slot="leadingIcon">key</Icon>
            <IconButton
                bind:innerHTML={icon}
                on:click={toggleVisibility}
                class="material-icons"
                slot="trailingIcon"
                style="outline: none;"
            >
                {icon}
            </IconButton>
        </Textfield>

        <div class="mdc-typography--headline4" style="padding-bottom: 1rem;">
            Don't have an account?
            <Link to="/signup" style="color: red;">Sign up here</Link>
        </div>

        <Button
            disabled={isButtonDisabled}
            color="primary"
            class="button"
            variant="raised"
            type="submit"
            style="width: 100%;"
            on:click={handleLogin}
            >
            <Label>Log In</Label>
        </Button>
    </div>
</div>

<style>
  .container {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100vh;
  }
</style>

<script lang="ts">
    import Button, { Label } from '@smui/button';
    import IconButton from '@smui/icon-button';
    import Textfield from '@smui/textfield';
    import Icon from '@smui/textfield/icon';
    import { onMount } from 'svelte';
    import { Link, useNavigate } from 'svelte-navigator';
    import type { ISignIn } from '../store/auth.store';
    import authStore, { jwtStore } from '../store/auth.store';
    import notifStore from '../store/notification.store';
    import type { AuthRes } from '../types/ifaces';
    import { axiosPost } from '../utils/api.utils';

    const signIn: ISignIn = {
        email: "",
        password: "",
    }
    const navigate = useNavigate();

    let isVisible = false;
    $: icon = isVisible ? "visibility" : "visibility_off";
    $: type = isVisible ? "text" : "password";
    $: isPasswordValid = signIn.password.trim().length > 0;
    $: isEmailValid = signIn.email.trim().length > 0;
    $: isButtonDisabled = !isPasswordValid || !isEmailValid;

    const toggleVisibility = () => {
        isVisible = !isVisible;
    };

    const handleLogin = async () => {
        const response = await axiosPost<AuthRes, ISignIn>("/api/auth/login", signIn);

        if (response.error || !response.data) {
            $notifStore.open(response.error ?? 'Error Logging in', 'error');
            return;
        }

        $authStore = response.data.content.user;
        $jwtStore = response.data.content.token;

        $notifStore.open('Successfully logged in', 'success');

        try {
            localStorage.setItem("user", JSON.stringify($authStore));
            localStorage.setItem("jwt", $jwtStore);
        } catch (err) {
            $notifStore.open(`Error saving auth in local storage: ${err.message}`, 'error');
        }

        navigate('/');
    };

    onMount(() => {
        if ($authStore !== null) {
            navigate('/');
        }
    });
</script>