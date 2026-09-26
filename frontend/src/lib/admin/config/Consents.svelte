<script lang="ts">
    import { onMount } from 'svelte';
    import Button from '$lib/button/Button.svelte';
    import Input from '$lib/form/Input.svelte';
    import InputCheckbox from '$lib/form/InputCheckbox.svelte';
    import Modal from '$lib/Modal.svelte';
    import SearchBar from '$lib/search_bar/SearchBar.svelte';
    import { fetchDelete, fetchGet, fetchPost } from '$api/fetch';
    import { fetchSearchServer } from '$utils/search';
    import { formatDateFromTs } from '$utils/helpers';
    import type { ConsentDoc, ConsentDocRequest, UserConsent } from '$api/types/consents';
    import type { UserResponseSimple } from '$api/types/user';

    let docs: ConsentDoc[] = $state([]);
    let error = $state('');

    // editor state
    let showEditor = $state(false);
    let editId = $state('');
    let editTitle = $state('');
    let editUrl = $state('');
    let editRequired = $state(false);
    let editReconfirm = $state('none');
    let editEnabled = $state(true);
    let editVersion = $state(0);

    // user status
    let searchValue = $state('');
    let searchOptions: UserResponseSimple[] = $state([]);
    let selectedId = $state('');
    let userConsents: UserConsent[] = $state([]);
    let statusLoaded = $state(false);

    onMount(() => getDocs());

    $effect(() => {
        searchUser();
    });

    async function getDocs() {
        error = '';
        let res = await fetchGet<ConsentDoc[]>('/auth/v1/consents');
        if (res.body) {
            docs = res.body;
        } else {
            error = res.error?.message || 'Error';
        }
    }

    function openCreate() {
        editId = '';
        editTitle = '';
        editUrl = '';
        editRequired = false;
        editReconfirm = 'none';
        editEnabled = true;
        editVersion = 0;
        showEditor = true;
    }

    function openEdit(doc: ConsentDoc) {
        editId = doc.id;
        editTitle = doc.title;
        editUrl = doc.url;
        editRequired = doc.required;
        editReconfirm = doc.reconfirm;
        editEnabled = doc.enabled;
        editVersion = doc.version;
        showEditor = true;
    }

    async function saveDoc() {
        error = '';
        if (!editId.trim() || !editTitle.trim() || !editUrl.trim()) {
            error = 'id, title and url are required';
            return;
        }
        let payload: ConsentDocRequest = {
            id: editId,
            title: editTitle,
            url: editUrl,
            required: editRequired,
            reconfirm: editReconfirm,
            enabled: editEnabled,
        };
        let res = await fetchPost<undefined>('/auth/v1/consents', payload);
        if (res.error) {
            error = res.error.message || 'Error';
            return;
        }
        showEditor = false;
        await getDocs();
    }

    async function disableDoc(id: string) {
        error = '';
        let res = await fetchDelete<undefined>('/auth/v1/consents/' + encodeURIComponent(id));
        if (res.error) {
            error = res.error.message || 'Error';
            return;
        }
        await getDocs();
    }

    async function searchUser() {
        if (searchValue.length < 3) {
            searchOptions = [];
            return;
        }
        let res = await fetchSearchServer<UserResponseSimple[]>({
            ty: 'user',
            idx: 'email',
            q: searchValue,
        });
        if (res.body) {
            searchOptions = res.body;
        }
    }

    async function loadStatus(id: string) {
        selectedId = id;
        statusLoaded = false;
        error = '';
        let res = await fetchGet<UserConsent[]>('/auth/v1/consents/user/' + encodeURIComponent(id));
        if (res.body) {
            userConsents = res.body;
        } else {
            userConsents = [];
        }
        statusLoaded = true;
    }

    async function revoke(cid: string) {
        error = '';
        let res = await fetchPost<undefined>(
            `/auth/v1/consents/user/${encodeURIComponent(selectedId)}/consent/${encodeURIComponent(cid)}/revoke`,
        );
        if (res.error) {
            error = res.error.message || 'Error';
            return;
        }
        await loadStatus(selectedId);
    }

    function fmtTs(ts?: number | null): string {
        if (!ts) {
            return '-';
        }
        return formatDateFromTs(ts);
    }
</script>

<div class="container">
    <div class="head">
        <h2>Consents</h2>
        <Button ariaLabel="Add new consent document" onclick={openCreate}>+ Add</Button>
    </div>

    {#if error}
        <div class="err">
            {error}
        </div>
    {/if}

    {#if docs.length === 0}
        <p class="empty">No consent documents configured yet.</p>
    {/if}

    <div class="list">
        {#each docs as doc (doc.id)}
            <div class="item">
                <div class="itemMain">
                    <div class="itemTitle">{doc.title}</div>
                    <div class="itemMeta">
                        <span class="mono">{doc.id}</span>
                        <span>v{doc.version}</span>
                        {#if doc.required}
                            <span class="tag">required</span>
                        {/if}
                        {#if !doc.enabled}
                            <span class="tag disabled">disabled</span>
                        {/if}
                        <span class="tag">{doc.reconfirm}</span>
                        <span>updated {fmtTs(doc.updated_ts)}</span>
                    </div>
                    <div class="itemMeta">
                        <a href={doc.url} target="_blank" rel="noreferrer">{doc.url}</a>
                    </div>
                </div>
                <div class="itemActions">
                    <Button
                        ariaLabel="Edit consent document"
                        level={2}
                        onclick={() => openEdit(doc)}
                    >
                        Edit
                    </Button>
                    {#if doc.enabled}
                        <Button
                            ariaLabel="Disable consent document"
                            level={2}
                            onclick={() => disableDoc(doc.id)}
                        >
                            Disable
                        </Button>
                    {/if}
                </div>
            </div>
        {/each}
    </div>

    <hr />

    <div class="userStatus">
        <h3>User Status</h3>
        <SearchBar placeholder="E-Mail" bind:value={searchValue} />
        {#if searchOptions.length > 0}
            <div class="searchResults">
                {#each searchOptions as u (u.id)}
                    <button class="searchResult" type="button" onclick={() => loadStatus(u.id)}>
                        {u.email} <span class="mono">({u.id})</span>
                    </button>
                {/each}
            </div>
        {/if}

        {#if selectedId}
            <h4>Acceptances for <span class="mono">{selectedId}</span></h4>
            {#if statusLoaded && userConsents.length === 0}
                <p class="empty">No consent acceptances found.</p>
            {/if}
            {#if statusLoaded}
                <div class="list">
                    {#each userConsents as uc (uc.consent_id + uc.version)}
                        <div class="item">
                            <div class="itemMain">
                                <div class="itemTitle">
                                    <span class="mono">{uc.consent_id}</span> v{uc.version}
                                </div>
                                <div class="itemMeta">
                                    <span>accepted {fmtTs(uc.accept_ts)}</span>
                                    <span class="mono">{uc.location}</span>
                                    {#if uc.withdrawn_at}
                                        <span class="tag">withdrawn {fmtTs(uc.withdrawn_at)}</span>
                                    {/if}
                                </div>
                            </div>
                            {#if !uc.withdrawn_at}
                                <div class="itemActions">
                                    <Button
                                        ariaLabel={`Revoke consent ${uc.consent_id}`}
                                        level={2}
                                        onclick={() => revoke(uc.consent_id)}
                                    >
                                        Revoke
                                    </Button>
                                </div>
                            {/if}
                        </div>
                    {/each}
                </div>
            {/if}
        {/if}
    </div>
</div>

{#if showEditor}
    <Modal bind:showModal={showEditor} strict>
        <h3>{editId ? `Edit: ${editId}` : 'New consent document'}</h3>
        <Input label="id" bind:value={editId} disabled={!!editId} />
        <Input label="title" bind:value={editTitle} />
        <Input label="url" bind:value={editUrl} />
        <div class="row">
            <label>
                <InputCheckbox bind:checked={editRequired} ariaLabel="required" />
                required
            </label>
            <label>
                <InputCheckbox bind:checked={editEnabled} ariaLabel="enabled" />
                enabled
            </label>
        </div>
        <div class="row">
            <label>
                reconfirm:
                <select bind:value={editReconfirm}>
                    <option value="login">login</option>
                    <option value="email">email</option>
                    <option value="none">none</option>
                </select>
            </label>
        </div>
        {#if error}
            <div class="err">
                {error}
            </div>
        {/if}
        <div class="row">
            <Button ariaLabel="Save consent document" onclick={saveDoc}>Save</Button>
            <Button ariaLabel="Cancel" level={2} onclick={() => (showEditor = false)}>
                Cancel
            </Button>
        </div>
    </Modal>
{/if}

<style>
    .container {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
    }

    .head {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .err {
        color: hsl(var(--error));
    }

    .empty {
        color: hsla(var(--text) / 0.6);
    }

    .list {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .item {
        display: flex;
        justify-content: space-between;
        align-items: center;
        gap: 0.75rem;
        padding: 0.5rem 0.75rem;
        border-radius: 5px;
        border: 1px solid hsl(var(--bg-high));
        background: hsla(var(--bg-high) / 0.25);
    }

    .itemMain {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
    }

    .itemTitle {
        font-weight: 600;
    }

    .itemMeta {
        display: flex;
        gap: 0.75rem;
        font-size: 0.8rem;
        color: hsla(var(--text) / 0.7);
        flex-wrap: wrap;
    }

    .itemActions {
        display: flex;
        gap: 0.5rem;
    }

    .tag {
        padding: 0 0.35rem;
        border-radius: 3px;
        background: hsla(var(--bg-high) / 0.6);
        font-size: 0.75rem;
    }

    .tag.disabled {
        color: hsl(var(--error));
    }

    .mono {
        font-family: var(--font-mono);
        font-size: 0.85em;
    }

    .searchResults {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
        margin-top: 0.5rem;
    }

    .searchResult {
        text-align: left;
        padding: 0.35rem 0.5rem;
        border-radius: 3px;
        background: hsla(var(--bg-high) / 0.4);
        cursor: pointer;
    }

    .row {
        display: flex;
        gap: 1rem;
        align-items: center;
        margin-top: 0.75rem;
    }

    hr {
        border: none;
        border-top: 1px solid hsl(var(--bg-high));
    }
</style>
