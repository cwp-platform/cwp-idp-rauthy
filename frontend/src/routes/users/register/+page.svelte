<script lang="ts">
    import { useI18n } from '$state/i18n.svelte';
    import RegisterForm from '$lib/RegisterForm.svelte';
    import Main from '$lib5/Main.svelte';
    import ContentCenter from '$lib5/ContentCenter.svelte';
    import ThemeSwitch from '$lib5/ThemeSwitch.svelte';
    import LangSelector from '$lib5/LangSelector.svelte';

    let t = useI18n();
</script>

<svelte:head>
    <title>{t?.register.register || 'Register'}</title>
</svelte:head>

<Main>
    <ContentCenter>
        <RegisterForm />
    </ContentCenter>

    <ThemeSwitch absolute />
    <LangSelector absolute />
</Main>
