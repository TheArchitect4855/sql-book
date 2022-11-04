<script>
    import { invoke } from "@tauri-apps/api/tauri";


	let error = '';

	async function onSubmit(e) {
		e.preventDefault();

		const form = new FormData(e.target);
		const scheme = form.get('driver');
		const user = form.get('user');
		const password = form.get('password');
		const host = form.get('host');
		const port = form.get('port');
		const schema = form.get('schema');

		let uri = `${scheme}://`;
		if(user) {
			uri += user;
			if(password) uri += `:${password}`;
			uri += '@';
		}

		uri += host;
		if(port) uri += `:${port}`;
		if(schema) uri += `/${schema}`;

		const info = {
			name: form.get('name'),
			uri,
			driver: scheme,
		};

		try {
			await invoke('add_connection', {
				info,
			});

			window.close();
		} catch(e) {
			error = e.toString();
		}
	}
</script>

<form on:submit={ onSubmit }>
	<section>
		<label for="name-input">Name:</label>
		<input id="name-input" name="name" type="text" required />
	</section>
	<section>
		<label for="driver-input">Driver:</label>
		<select id="driver-input" name="driver">
			<option value="mysql">MySQL</option>
		</select>
	</section>
	<section>
		<label for="host-input">Host:</label>
		<input id="host-input" name="host" type="text" required />
	</section>
	<section>
		<label for="port-input">Port:</label>
		<input id="port-input" name="port" type="text" pattern="^\d+$" />
	</section>
	<section>
		<label for="user-input">User:</label>
		<input id="user-input" name="user" type="text" />
	</section>
	<section>
		<label for="password-input">Password:</label>
		<input id="password-input" name="password" type="password" />
	</section>
	<section>
		<label for="default-schema-input">Default Schema:</label>
		<input id="default-schema-input" name="schema" type="text" />
	</section>
	<section>
		<button type="submit">Create</button>
	</section>
	<section>
		<p class="error">{ error }</p>
	</section>
</form>

<style>
	button {
		background-color: var(--highlight-color);
		border: none;
		border-radius: 4px;
		box-shadow: 0 0 4px 0 #aaa;
		color: #fff;
		padding: 0.5ch 1ch;
		transition: box-shadow 0.2s, transform 0.2s;
	}

	button:hover {
		box-shadow: 0 0 8px 0 #aaa;
		transform: scale(1.02);
	}

	section {
		align-items: center;
		display: flex;
		gap: 1ch;
		justify-content: flex-end;
		margin: 1em;
	}

	section > label {
		flex: 1;
	}
</style>
