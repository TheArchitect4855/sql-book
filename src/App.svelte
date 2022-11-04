<script>
    import Home from "./lib/Home.svelte";
	import NewConnection from "./lib/NewConnection.svelte";
</script>

<main>
	{#if window.location.pathname == '/'}
		<Home />
	{:else if window.location.pathname == '/add-connection'}
		<NewConnection />
	{:else}
		<div class="panel">
			INVALID WINDOW { window.location.pathname }
		</div>
	{/if}
</main>

<style>
	main {
		overflow: hidden;

		position: absolute;
		left: 0;
		right: 0;
		top: 0;
		bottom: 0;

		display: flex;
		flex-direction: row;
	}
</style>
