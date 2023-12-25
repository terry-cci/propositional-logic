<script lang="ts">
	import FunctionInput from '$lib/components/FunctionInput.svelte';
	import { OPERATORS, toPreview, type Operator } from '$lib/toPreview';
	import { invoke } from '@tauri-apps/api/tauri';

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

<form on:submit|preventDefault={generateTable}>
	<FunctionInput
		bind:value={functionText}
		on:clear={() => {
			truthTable = null;
		}}
	/>

	<div class="preview">
		<h3>Function Preview</h3>

		<div class="horizontal-overflow-container">
			<p>
				{#if functionText}
					{toPreview(functionText)}
				{:else}
					<small>(Waiting for input...)</small>
				{/if}
			</p>
		</div>
	</div>
	<button type="submit" disabled={!functionText}> Generate Truth Table </button>

	{#if truthTable}
		<div>
			<h3>Truth Table</h3>

			<div class="horizontal-overflow-container">
				<table>
					<thead>
						<tr>
							{#each truthTable[0] as cell}
								<th scope="col">
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
