<script lang="ts">
    import IconButton from '@smui/icon-button';
    import Textfield from '@smui/textfield';
    import Icon from '@smui/textfield/icon';
    import { createEventDispatcher } from 'svelte';

    export let style: string = 'width: 100%; margin-bottom: 1rem;';
    export let password: string;
    export let invalid: boolean = false;

    const dispatch = createEventDispatcher();

    let isVisible = false;
    $: icon = isVisible ? 'visibility' : 'visibility_off';
    $: type = isVisible ? 'text' : 'password';

    const toggleVisibility = () => {
        isVisible = !isVisible;
    };

    const handleChange = () => {
        dispatch('change', password);
    };
</script>

<Textfield
    variant="filled"
    bind:value={password}
    label="Password"
    required
    onchange={handleChange}
    {type}
    {style}
    {invalid}
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
