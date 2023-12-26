<script lang="ts">
	import ErrorDialog from '$lib/components/ErrorDialog.svelte';
	import FunctionInput from '$lib/components/FunctionInput.svelte';
	import { toPreview } from '$lib/toPreview';
	import { invoke } from '@tauri-apps/api/tauri';

	let functionText = '';

	let truthTable: [string, boolean][][] | null = null;
	let errorMsg: string | null = null;
	let errorDialog: ErrorDialog;

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
	<button type="submit" disabled={!functionText}> Generate Truth Table </button>

	{#if truthTable}
		<div>
			<h3>Truth Table</h3>

			{#if truthTable[0].length}
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
			{:else}
				<p>(Empty)</p>
			{/if}
		</div>
	{/if}
</form>

<ErrorDialog bind:this={errorDialog} {errorMsg} />
