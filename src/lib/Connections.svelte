<script>
	import { invoke } from "@tauri-apps/api/tauri";
	import { onMount } from "svelte";
	import { WebviewWindow } from '@tauri-apps/api/window';

	export let selectedConnection = null;

	let connections = null;
	let selectedConnIdx = 0;

	$: selectedConnection = (connections && connections[selectedConnIdx]) || null;
	onMount(() => {
		getConnections();
	});

	function openCreateConnectionWindow() {
		const view = new WebviewWindow('create-connection', {
			alwaysOnTop: true,
			center: true,
			resizable: false,
			title: 'Create New Connection',
			url: '/add-connection',
		});

		view.onCloseRequested(getConnections);
	}

	async function getConnections() {
		connections = await invoke('get_connections');
	}

	async function deleteConnection(e, id) {
		e.stopPropagation();
		connections = await invoke('remove_connection', { id });
	}
</script>

<header>
	<b>Connections</b>
</header>
<div>
	<hr />
	{#if connections}
		{#each connections as conn, i}
			<div
				class="connection"
				data-selected={ selectedConnIdx == i }
				on:click="{ () => selectedConnIdx = i }"
				on:keypress="{ console.dir }"
			>
				{ conn.name }
				<small class="subtitle">{ conn.host }</small>
				<button on:click={ (e) => deleteConnection(e, i) } class="trash-icon">
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
						<path fill-rule="evenodd" d="M8.75 1A2.75 2.75 0 006 3.75v.443c-.795.077-1.584.176-2.365.298a.75.75 0 10.23 1.482l.149-.022.841 10.518A2.75 2.75 0 007.596 19h4.807a2.75 2.75 0 002.742-2.53l.841-10.52.149.023a.75.75 0 00.23-1.482A41.03 41.03 0 0014 4.193V3.75A2.75 2.75 0 0011.25 1h-2.5zM10 4c.84 0 1.673.025 2.5.075V3.75c0-.69-.56-1.25-1.25-1.25h-2.5c-.69 0-1.25.56-1.25 1.25v.325C8.327 4.025 9.16 4 10 4zM8.58 7.72a.75.75 0 00-1.5.06l.3 7.5a.75.75 0 101.5-.06l-.3-7.5zm4.34.06a.75.75 0 10-1.5-.06l-.3 7.5a.75.75 0 101.5.06l.3-7.5z" clip-rule="evenodd" />
					</svg>
				</button>
			</div>
		{:else}
			<i>No connections</i>
		{/each}
	{:else}
		<i>Loading...</i>
	{/if}
	<button on:click={ openCreateConnectionWindow }>
		<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="add-icon">
			<path d="M10.75 4.75a.75.75 0 00-1.5 0v4.5h-4.5a.75.75 0 000 1.5h4.5v4.5a.75.75 0 001.5 0v-4.5h4.5a.75.75 0 000-1.5h-4.5v-4.5z" />
		</svg>
		New Connection
	</button>
</div>

<style>
	.connection {
		cursor: pointer;
		user-select: none;
		padding: 0.5em 1em;

		display: flex;
		gap: 1ch;
		align-items: center;
		justify-content: space-between;
	}

	.connection[data-selected="true"] {
		background-color: var(--highlight-color);
	}

	.connection[data-selected="true"] > .subtitle {
		color: var(--text-color);
	}

	.add-icon {
		width: 2.5ch;
		height: 2.5ch;
	}

	.trash-icon {
		width: 1.5ch;
		height: 1.5ch;
		padding: 0;
	}

	.trash-icon:hover {
		background-color: inherit;
	}

	button {
		background-color: transparent;
		border: none;
		box-sizing: border-box;
		padding: 0.5em 1em;
		transition: background-color 0.2s;
		width: 100%;

		display: flex;
		align-items: center;
		gap: 1ch;
	}

	button:hover {
		background-color: var(--background-secondary-color);
	}

	header {
		margin: 1em 0.5em;
	}

	svg {
		width: 1.5ch;
		height: 1.5ch;
	}
</style>
