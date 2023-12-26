<script lang="ts">
	import ErrorDialog from '$lib/components/ErrorDialog.svelte';
	import FunctionInput from '$lib/components/FunctionInput.svelte';
	import { toPreview } from '$lib/toPreview';
	import { invoke } from '@tauri-apps/api/tauri';

	let functionText = new URLSearchParams(location.search).get('f') ?? '';
	let errorDialog: ErrorDialog;
	let errorMsg: string | null = null;

	let normalForms: [string, string] | null = null;

	async function normaliseFunction() {
		try {
			normalForms = null;

			const postfix = await invoke<string[]>('parse_function_text', { text: functionText });
			console.debug(postfix);

			const tableRows = await invoke<[string, boolean][][]>('evaluate_truth_table', {
				postfixExpression: postfix.join('')
			});
			console.debug(tableRows);

			const normalised = await invoke<[string, string]>('normalise_function', {
				variables: tableRows[0]
					.map(([variable, _]) => variable)
					.filter((v) => v.length === 1 && v !== 'T' && v !== 'F'),
				rowValues: tableRows.reduce((obj: { [key: number]: boolean }, row, idx) => {
					if (row.length) obj[idx] = row[row.length - 1][1];
					return obj;
				}, {})
			});
			normalForms = normalised;
			console.debug(normalForms);
		} catch (error) {
			normalForms = null;
			if (typeof error === 'string') errorMsg = error;
			errorDialog.showModal();
		}
	}
</script>

<h2>Normalise Function</h2>
<form on:submit|preventDefault={normaliseFunction}>
	<FunctionInput bind:value={functionText} />

	<button type="submit" disabled={!functionText}> Normalise Function </button>
</form>

{#if normalForms}
	<div class="preview">
		<h3>Output</h3>

		<h4>Disjunctive Normal Form</h4>
		<div class="horizontal-overflow-container">
			<p>
				{toPreview(normalForms[0])}
			</p>
			<div class="input-text">{normalForms[0] || '(Empty)'}</div>
		</div>
		<button
			type="button"
			class="send-btn"
			on:click={() => {
				location.href = '/f2tt?f=' + encodeURIComponent(normalForms?.[0] ?? '');
			}}
		>
			Send to Truth Table
		</button>

		<h4>Conjunctive Normal Form</h4>
		<div class="horizontal-overflow-container">
			<p>
				{toPreview(normalForms[1])}
			</p>
			<div class="input-text">{normalForms[1] || '(Empty)'}</div>
		</div>
		<button
			type="button"
			class="send-btn"
			on:click={() => {
				location.href = '/f2tt?f=' + encodeURIComponent(normalForms?.[1] ?? '');
			}}
		>
			Send to Truth Table
		</button>
	</div>
{/if}

<ErrorDialog bind:this={errorDialog} {errorMsg} />
