<script lang="ts">
    import { onMount } from 'svelte';

    // Single entry point: the unified login/register page lives at /auth/v1/oidc/authorize
    // and works without any OIDC client params (defaults to the account client).
    onMount(() => {
        window.location.replace('/auth/v1/oidc/authorize');
    });
</script>

<svelte:head>
    <title>Rauthy</title>
</svelte:head>
