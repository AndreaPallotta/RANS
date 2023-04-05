<div class="container">
    <div style="background-color: rgb(207, 205, 204, 0.4); padding: 3rem; border-radius: 10px">
        <Textfield
            variant="filled"
            bind:value={signUp.first_name}
            label="First Name"
            style="width: 100%; margin-bottom: 1rem;"
            required
            invalid={!isFirstNameValid}
        >
            <Icon class="material-icons" slot="leadingIcon">person</Icon>
        </Textfield>

        <Textfield
            variant="filled"
            bind:value={signUp.last_name}
            label="Last Name"
            style="width: 100%; margin-bottom: 1rem;"
            required
            invalid={!isLastNameValid}
        >
            <Icon class="material-icons" slot="leadingIcon">person</Icon>
        </Textfield>

        <Textfield
            variant="filled"
            bind:value={signUp.email}
            label="Email"
            style="width: 100%; margin-bottom: 1rem;"
            required
            invalid={!isEmailValid}
        >
            <Icon class="material-icons" slot="leadingIcon">email</Icon>
        </Textfield>

        <Textfield
            variant="filled"
            bind:value={signUp.password}
            label="Password"
            {type}
            style="width: 100%;"
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
            <HelperText slot="helper">Password length must be at least 8</HelperText>
        </Textfield>

        <div class="mdc-typography--headline4" style="padding-bottom: 1rem;">
            Don't have an account?
            <Link to="/login" style="color: red;">Login now</Link>
        </div>

        <Button
            disabled={isButtonDisabled}
            color="primary"
            class="button"
            variant="raised"
            type="submit"
            style="width: 100%;"
            on:click={handleSignUp}
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
    import HelperText from '@smui/textfield/helper-text';
    import Icon from '@smui/textfield/icon';
    import { onMount } from 'svelte';
    import { Link, useNavigate } from 'svelte-navigator';
    import type { ISignUp } from '../store/auth.store';
    import authStore from '../store/auth.store';
    import notifStore from '../store/notification.store';
    import type { IUser } from '../types/models';
    import { axiosPost } from '../utils/api.utils';

    const signUp: ISignUp = {
        first_name: "",
        last_name: "",
        email: "",
        password: "",
    }
    const navigate = useNavigate();

    let isVisible = false;
    $: icon = isVisible ? "visibility" : "visibility_off";
    $: type = isVisible ? "text" : "password";
    $: isFirstNameValid = signUp.first_name.trim().length > 0;
    $: isLastNameValid = signUp.last_name.trim().length > 0;
    $: isPasswordValid = signUp.password.trim().length >= 8;
    $: isEmailValid = /^\w+([\.-]?\w+)*@\w+([\.-]?\w+)*(\.\w{2,3})+$/.test(signUp.email);
    $: isButtonDisabled = !isFirstNameValid || !isLastNameValid || !isPasswordValid || !isEmailValid;

    const toggleVisibility = () => {
        isVisible = !isVisible;
    };

    const handleSignUp = async () => {
        const response = await axiosPost<IUser, ISignUp>("/api/auth/signup", signUp);

        if (response.error || !response.data) {
            $notifStore.open(response.error ?? 'Error Logging in', 'error');
            return;
        }

        $authStore = response.data.content as IUser;
        $notifStore.open('Successfully signed up', 'success');

        navigate('/');
    };

    onMount(() => {
        if ($authStore !== null) {
            navigate('/');
        }
    });
</script>