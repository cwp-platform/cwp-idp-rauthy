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
</script>

{#if consents.length > 0}
    <div class="consents">
        <div class="consentsTitle">{t.register.consentsTitle}</div>
        {#each consents as consent (consent.id)}
            <label class="consentRow">
                <input
                    type="checkbox"
                    bind:checked={accepted[consent.id]}
                    required={consent.required}
                />
                <span>
                    {consent.title}
                    <a href={consent.url} target="_blank" rel="noreferrer">
                        {t.register.consentsOpen}
                    </a>
                    {#if consent.required}
                        <span class="consentRequired">
                            ({t.register.consentsRequiredShort})
                        </span>
                    {/if}
                </span>
            </label>
        {/each}
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
</style>
