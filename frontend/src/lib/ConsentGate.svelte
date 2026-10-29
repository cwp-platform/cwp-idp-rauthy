<script lang="ts">
    import Button from '$lib5/button/Button.svelte';
    import { useI18n } from '$state/i18n.svelte';
    import type { ConsentPendingItem } from '$api/types/consents';

    let {
        pendingConsents,
        choices = $bindable({}),
        error = '',
        isLoading = false,
        onConfirm,
    }: {
        pendingConsents: ConsentPendingItem[];
        choices: Record<string, boolean>;
        error: string;
        isLoading: boolean;
        onConfirm: () => void;
    } = $props();

    let t = useI18n();

    let requiredConsents = $derived(pendingConsents.filter(c => c.required));
    let optionalConsents = $derived(pendingConsents.filter(c => !c.required));
</script>

{#if pendingConsents.length > 0}
    <div class="consents">
        <div class="consentsTitle">{t.authorize.consentsTitle}</div>
        <p class="consentsInfo">{t.authorize.consentsInfo}</p>
        {#each requiredConsents as consent (consent.id)}
            <label class="consentRow">
                <input
                    type="checkbox"
                    bind:checked={choices[consent.id]}
                    disabled={consent.required}
                />
                <a href={consent.url} target="_blank" rel="noreferrer">{consent.title}</a>
                <span class="consentRequired">({t.authorize.consentsRequiredShort})</span>
            </label>
        {/each}
        {#if optionalConsents.length > 0}
            <div class="consentsDivider" role="separator"></div>
            {#each optionalConsents as consent (consent.id)}
                <label class="consentRow optional">
                    <input type="checkbox" bind:checked={choices[consent.id]} />
                    <a href={consent.url} target="_blank" rel="noreferrer">{consent.title}</a>
                </label>
            {/each}
        {/if}
        {#if error}
            <div class="errMsg">
                {error}
            </div>
        {/if}
        <div class="btn flex-col">
            <Button ariaLabel={t.authorize.consentsConfirm} onclick={onConfirm} {isLoading}>
                {t.authorize.consentsConfirm}
            </Button>
        </div>
    </div>
{/if}

<style>
    .consents {
        margin-top: 1rem;
        padding: 0.75rem;
        border-radius: 5px;
        border: 1px solid hsl(var(--bg-high));
        background: hsla(var(--bg-high) / 0.25);
    }

    .consentsTitle {
        font-size: 0.9rem;
        font-weight: 600;
        margin-bottom: 0.25rem;
    }

    .consentsInfo {
        margin: 0 0 0.5rem 0;
        font-size: 0.8rem;
        color: hsla(var(--text) / 0.7);
    }

    .consentsDivider {
        height: 1px;
        margin: 0.75rem 0;
        background: hsla(var(--text) / 0.2);
    }

    .consentRow {
        display: flex;
        gap: 0.5rem;
        align-items: flex-start;
        margin-bottom: 0.5rem;
        font-size: 0.85rem;
    }

    .consentRow input[type='checkbox'] {
        width: auto;
        margin: 0;
        padding: 0;
        border: none;
        background: none;
        flex: 0 0 auto;
        accent-color: hsl(var(--action));
    }

    .consentRow.optional {
        opacity: 0.9;
    }

    .consentRow a {
        color: hsl(var(--action));
    }

    .consentRequired {
        color: hsla(var(--text) / 0.6);
    }

    .errMsg {
        margin: 0.5rem 0;
        max-width: 18rem;
        text-wrap: wrap;
        color: hsl(var(--error));
    }

    .btn {
        margin: 0.8rem 0 0.5rem 0;
        display: flex;
    }

    .flex-col {
        display: flex;
        flex-direction: column;
    }
</style>
