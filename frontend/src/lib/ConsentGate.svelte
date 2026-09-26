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
</script>

{#if pendingConsents.length > 0}
    <div class="consents">
        <div class="consentsTitle">{t.authorize.consentsTitle}</div>
        <p class="consentsInfo">{t.authorize.consentsInfo}</p>
        {#each pendingConsents as consent (consent.id)}
            <label class="consentRow">
                <input
                    type="checkbox"
                    bind:checked={choices[consent.id]}
                    disabled={consent.required}
                />
                <span>
                    {consent.title}
                    <a href={consent.url} target="_blank" rel="noreferrer">
                        {t.authorize.consentsOpen}
                    </a>
                    {#if consent.required}
                        <span class="consentRequired">
                            ({t.authorize.consentsRequiredShort})
                        </span>
                    {/if}
                </span>
            </label>
        {/each}
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

    .consentRow {
        display: flex;
        gap: 0.5rem;
        align-items: flex-start;
        margin-bottom: 0.5rem;
        font-size: 0.85rem;
    }

    .consentRow a {
        margin-left: 0.25rem;
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
