<script>
	import Connections from "./Connections.svelte";
	import Notebook from "./Notebook.svelte";

	let conn;
</script>

<section>
	<Connections bind:selectedConnection={ conn } />
</section>
<section class="notebook">
	<Notebook connection={ conn } />
</section>

<style>
	.notebook {
		border-left: 1px solid var(--border-color);
		box-sizing: border-box;
		flex: 1;
		min-width: 0;
		padding: 0 1em;
		position: relative;
	}
</style>
