<script lang="ts">
    import { type Snippet } from 'svelte';
    import { useI18n } from '$state/i18n.svelte';

    let {
        activeTab = 'login',
        isRegOpen = false,
        onSwitch,
        login,
        register,
    }: {
        activeTab: 'login' | 'register';
        isRegOpen: boolean;
        onSwitch: (tab: 'login' | 'register') => void;
        login: Snippet;
        register: Snippet;
    } = $props();

    let t = useI18n();
</script>

{#if isRegOpen}
    <div class="authTabs">
        <div class="tabs" role="tablist">
            <button
                class="tab {activeTab === 'login' ? 'active' : ''}"
                role="tab"
                aria-selected={activeTab === 'login'}
                onclick={() => onSwitch('login')}
            >
                {t.authorize.login}
            </button>
            <button
                class="tab {activeTab === 'register' ? 'active' : ''}"
                role="tab"
                aria-selected={activeTab === 'register'}
                onclick={() => onSwitch('register')}
            >
                {t.authorize.signUp}
            </button>
        </div>
        {#if activeTab === 'register' && isRegOpen}
            {@render register()}
        {:else}
            {@render login()}
        {/if}
    </div>
{:else}
    {@render login()}
{/if}

<style>
    .authTabs {
        display: flex;
        flex-direction: column;
        margin-top: 0.5rem;
    }

    .tabs {
        display: flex;
        gap: 0.5rem;
        margin-bottom: 1rem;
    }

    .tab {
        padding: 0.35rem 0.75rem;
        border-radius: 5px;
        border: 1px solid hsla(var(--text) / 0.2);
        background: hsl(var(--bg));
        color: hsl(var(--text));
        cursor: pointer;
        font-size: 0.9rem;
    }

    .tab.active {
        background: hsla(var(--action) / 0.12);
        border-color: hsl(var(--action));
        color: hsl(var(--text-high));
        font-weight: 600;
    }
</style>
