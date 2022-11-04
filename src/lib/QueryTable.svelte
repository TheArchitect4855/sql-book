<script>
	export let data;
</script>

<table>
	<thead>
		<tr>
			{#each data.cols as col}
				<th>{ col.name }</th>
			{/each}
		</tr>
	</thead>
	<tbody>
		{#each data.rows as row}
			<tr>
				{#each row.values as v}
					<td>{ v }</td>
				{/each}
			</tr>
		{/each}
	</tbody>
</table>

<style>
	table {
		background-color: var(--background-secondary-color);
		border-collapse: collapse;
		border-radius: 4px;
		min-width: max-content;
		padding: 0.25em;
	}

	td, th {
		border: 1px solid var(--border-color);
		padding: 0.25em 0.5em;
	}

	td:first-child {
		border-left: none;
	}

	td:last-child {
		border-right: none;
	}

	tr:first-child th {
		border-top: none;
	}

	tr:first-child th:first-child {
		border-left: none;
	}

	tr:first-child th:last-child {
		border-right: none;
	}

	tr:last-child td {
		border-bottom: none;
	}

	tr:last-child td:first-child {
		border-left: none;
	}

	tr:last-child td:last-child {
		border-right: none;
	}
</style>
