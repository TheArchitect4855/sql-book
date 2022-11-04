<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import { afterUpdate } from "svelte";
    import QueryTable from "./QueryTable.svelte";

	export let connection;

	let cid;
	let historyContainer;

	let history = {};

	$: {
		cid = connection?.id;
		if(!history[cid]) history[cid] = [];
	}

	afterUpdate(scrollHistory);

	async function onKeyDown(e) {
		if(e.code == 'Enter' && !e.shiftKey) {
			e.preventDefault();
			const query = e.target.innerText;
			if(!query.trim()) return;

			e.target.innerText = '';
			const hist = {
				query,
			};

			history[cid] = [ ...history[cid], hist ];

			try {
				const res = await invoke('query', { connId: connection.id, query, });
				hist.result = res;
			} catch(e) {
				hist.error = e;
			}

			history[cid] = [ ...history[cid] ];
		} else if(e.code == 'Tab') {
			e.preventDefault();

			const selection = window.getSelection();
			if(selection) {
				const node = selection.focusNode;
				const range = selection.getRangeAt(0);
				const offset = range.startOffset;
				const text = node.textContent;
				node.textContent = text.substring(0, range.startOffset)
					+ '\t' + text.substring(range.endOffset);

				selection.collapse(node, offset + 1);
			}
		}
	}

	function scrollHistory() {
		if(!historyContainer) return;

		historyContainer.scrollTop = historyContainer.scrollHeight + 50;
	}
</script>

{#if connection}
	<div class="content">
		<h1>{ connection.name }</h1>
		<hr />
		<div class="history" bind:this={ historyContainer }>
			{#each history[cid] as hist}
				<pre class="sql-input">{ hist.query }</pre>
				{#if hist.error}
					<p class="error">{ hist.error }</p>
				{:else if hist.result}
					<div class="h-scroll">
						<QueryTable data={ hist.result } />
					</div>
				{:else}
					<div class="spinner"></div>
				{/if}
				<hr />
			{/each}

			<pre class="sql-input" on:keydown={ onKeyDown } contenteditable></pre>
		</div>
	</div>
{:else}
	<div class="panel">
		Select a connection on the left panel
	</div>
{/if}

<style>
	.content {
		box-sizing: border-box;
		max-height: 100%;
		max-width: 100%;
		padding: 1em;
	}

	.h-scroll {
		max-width: 100%;
		overflow-x: auto;
		padding-bottom: 1em;
	}

	.history {
		box-sizing: border-box;
		max-height: 90vh;
		overflow-x: hidden;
		overflow-y: auto;
		padding: 0 1em 1em 0;
	}

	.sql-input {
		border: 2px solid var(--border-color);
		border-radius: 4px;
		box-sizing: border-box;
		display: block;
		font-family: monospace;
		outline: none;
		padding: 0.25em;
		resize: none;
		width: 100%;
	}
</style>
