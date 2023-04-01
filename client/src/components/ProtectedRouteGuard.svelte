<script>
	import { useLocation, useNavigate } from "svelte-navigator";
	import authStore from "../store/auth.store";

	const navigate = useNavigate();
	const location = useLocation();

	$: if ($authStore === null) {
		navigate("/login", {
			state: { from: $location.pathname },
			replace: true,
		});
	} else {
        navigate("/", {
			state: { from: $location.pathname },
			replace: true,
		});
    }
</script>

{#if $authStore !== null}
	<slot />
{/if}