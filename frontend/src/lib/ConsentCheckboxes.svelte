<script lang="ts">
    import { useI18n } from '$state/i18n.svelte';
    import type { ConsentDocPublic } from '$api/types/consents';

    let {
        consents,
        accepted = $bindable({}),
    }: {
        consents: ConsentDocPublic[];
        accepted: Record<string, boolean>;
    } = $props();

    let t = useI18n();

    let requiredConsents = $derived(consents.filter(c => c.required));
    let optionalConsents = $derived(consents.filter(c => !c.required));
</script>

{#if requiredConsents.length > 0 || optionalConsents.length > 0}
    <div class="consents">
        <div class="consentsTitle">{t.register.consentsTitle}</div>
        {#each requiredConsents as consent (consent.id)}
            <label class="consentRow">
                <input
                    type="checkbox"
                    bind:checked={accepted[consent.id]}
                    required={consent.required}
                />
                <a href={consent.url} target="_blank" rel="noreferrer">{consent.title}</a>
                <span class="consentRequired">({t.register.consentsRequiredShort})</span>
            </label>
        {/each}
        {#if optionalConsents.length > 0}
            <div class="consentsDivider" role="separator"></div>
            <div class="consentsOptional">{t.register.consentsOptional}</div>
            {#each optionalConsents as consent (consent.id)}
                <label class="consentRow optional">
                    <input type="checkbox" bind:checked={accepted[consent.id]} />
                    <a href={consent.url} target="_blank" rel="noreferrer">{consent.title}</a>
                </label>
            {/each}
        {/if}
    </div>
{/if}

<style>
    .consents {
        margin-top: 1rem;
        padding: 0.75rem;
        border-radius: var(--border-radius);
        background: hsla(var(--bg-high) / 0.5);
    }

    .consentsTitle {
        font-size: 0.9rem;
        font-weight: 600;
        margin-bottom: 0.5rem;
    }

    .consentsOptional {
        font-size: 0.85rem;
        color: hsla(var(--text) / 0.7);
        margin-bottom: 0.5rem;
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
</style>
