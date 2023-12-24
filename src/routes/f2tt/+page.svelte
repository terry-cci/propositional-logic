<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	type Operator = {
		displayText: string;
		inputText: string;
		name: string;
	};
	const OPERATORS: Operator[] = [
		{
			displayText: '¬',
			inputText: '!',
			name: 'Not'
		},
		{
			displayText: '∧',
			inputText: '&',
			name: 'And'
		},
		{
			displayText: '∨',
			inputText: '|',
			name: 'Or'
		},
		{
			displayText: '→',
			inputText: '^',
			name: 'If...then'
		},
		{
			displayText: '↔',
			inputText: '~',
			name: 'If and only if'
		}
	];

	function insertOperator(operator: Operator) {
		const startPosition = functionTextarea.selectionStart;
		const endPosition = functionTextarea.selectionEnd;

		functionTextarea.value =
			functionTextarea.value.substring(0, startPosition) +
			operator.inputText +
			functionTextarea.value.substring(endPosition);

		functionTextarea.focus();
		functionTextarea.selectionEnd = startPosition + 1;

		functionText = functionTextarea.value;
	}

	let functionTextarea: HTMLTextAreaElement;
	let functionText = '';

	function toPreview(rawText: string) {
		let previewText = rawText;
		OPERATORS.forEach((operator) => {
			previewText = previewText.replaceAll(operator.inputText, operator.displayText);
		});
		return previewText;
	}

	let truthTable: [string, boolean][][] | null = null;
	let errorMsg: string | null = null;
	let errorDialog: HTMLDialogElement;

	async function generateTable() {
		try {
			truthTable = null;
			const postfix = await invoke<string[]>('parse_function_text', { text: functionText });
			console.debug(postfix);
			const tableRows = await invoke<[string, boolean][][]>('evaluate_truth_table', {
				postfixExpression: postfix.join('')
			});
			truthTable = tableRows;
			console.debug(tableRows);
		} catch (error) {
			truthTable = null;
			if (typeof error === 'string') errorMsg = error;
			errorDialog.showModal();
		}
	}
</script>

<h2>Function to Truth Table</h2>

<form>
	<div>
		<label for="function-textarea">Input Propositional Function:</label>
		<textarea
			name="function"
			id="function-textarea"
			rows="2"
			placeholder="Input the function to be evaluated here..."
			bind:this={functionTextarea}
			bind:value={functionText}
		/>
		<div id="action-bar">
			<ul>
				{#each OPERATORS as operator}
					<li>
						<button
							type="button"
							title={operator.name}
							on:click={() => {
								insertOperator(operator);
							}}
						>
							{operator.displayText}
							<small> ({operator.inputText})</small>
						</button>
					</li>
				{/each}
			</ul>
			<button
				type="reset"
				title="Clear"
				on:click={() => {
					functionText = '';
				}}
			>
				Clear
			</button>
		</div>
	</div>

	<div id="preview">
		<h3>Function Preview</h3>
		<p>
			{#if functionText}
				{toPreview(functionText)}
			{:else}
				<small>(Waiting for input...)</small>
			{/if}
		</p>
	</div>
	<button type="submit" disabled={!functionText} on:click={generateTable}
		>Generate Truth Table</button
	>

	{#if truthTable}
		<div>
			<h3>Truth Table</h3>

			<div id="truth-table-container">
				<table>
					<thead>
						<tr>
							{#each truthTable[0] as cell}
								<th>
									{toPreview(cell[0])}
								</th>
							{/each}
						</tr>
					</thead>
					<tbody>
						{#each truthTable as row}
							<tr>
								{#each row as cell}
									<td class:active={cell[1]}>{cell[1] ? 'T' : 'F'}</td>
								{/each}
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		</div>
	{/if}
</form>

<dialog bind:this={errorDialog}>
	<h1>An Error has Occured</h1>

	<p>{errorMsg}</p>

	<form>
		<button type="submit" formmethod="dialog">Close</button>
	</form>
</dialog>

<style>
	textarea {
		font-family: 'Noto Sans Mono', sans-serif;
		font-size: large;
	}

	#action-bar {
		display: flex;
		justify-content: space-between;
	}

	ul {
		display: flex;
		list-style: none;
		padding-left: 0;
		gap: 0.5rem;
		margin: 0;
	}

	ul button {
		font-family: 'Noto Sans Math', sans-serif;
		font-size: 1.3rem;
		display: flex;
		flex-direction: column;
		align-items: center;
		line-height: 0.8em;
	}
	ul button small {
		color: var(--colour-4);
		font-size: small;
	}

	#preview p {
		margin: 0;
		text-align: center;
		font-family: 'Noto Sans Math', sans-serif;
		font-size: 1.5rem;
	}

	#preview small {
		font-size: small;
	}

	button[type='submit'] {
		font-weight: bold;
		font-size: 1rem;
		margin-top: 1em;
		width: max-content;
		margin-left: auto;
		margin-right: auto;
		padding: 0.5em 3em;
		background-color: var(--colour-3);
	}
	button[type='submit']:hover {
		background-color: var(--colour-2);
	}
	button[type='submit']:active {
		background-color: var(--colour-3);
	}
	button[type='submit']:disabled {
		background-color: var(--colour-3);
	}

	#truth-table-container {
		overflow-x: auto;
		margin: 0 -2.5rem;
		padding: 0 2.5rem;
		padding-bottom: 1rem;
	}

	table {
		width: max-content;
		border-collapse: collapse;
	}

	table td,
	table th {
		border: 1px solid var(--colour-4);
		padding: 0.25em 1em;
	}

	table th {
		background-color: var(--colour-3);
		color: var(--colour-5);
		font-family: 'Noto Sans Math', sans-serif;
	}

	table td.active {
		background-color: #91d6f1;
		font-weight: bold;
	}

	dialog h1 {
		font-size: 1.3rem;
	}

	dialog {
		border-radius: 0.5rem;
		box-shadow: 0 0 5px var(--colour-3);
		padding: 2rem 2.5rem;
	}

	dialog::backdrop {
		background-color: rgba(4, 57, 89, 0.8);
	}
</style>
