<script lang="ts">
	import { toPreview } from '$lib/toPreview';
	import { invoke } from '@tauri-apps/api/tauri';

	let variables: string[] = ['P', 'Q'];

	$: repeatedNamesExists = new Set(variables).size < variables.length;

	let rowValues: { [key: number]: boolean } = {};

	let functionText: string | null = null;

	async function generateFunction() {
		console.debug(rowValues);
		console.debug(variables);

		const output = await invoke<string>('generate_function', {
			rowValues,
			variables
		});

		functionText = output;
	}

	function addVariable() {
		if (variables.length >= 6) return;

		const lastVariableName = ((a) => a[a.length - 1] ?? 'O')(variables.filter((v) => v.length));

		let newVariableName = String.fromCharCode(
			Math.max(Math.min(lastVariableName.charCodeAt(0), 'Z'.charCodeAt(0)), 'A'.charCodeAt(0)) + 1
		);

		variables.push(newVariableName);
		variables = variables;

		rowValues = Object.keys(rowValues).reduce((newObj: { [key: number]: boolean }, idx) => {
			const row = parseInt(idx);
			newObj[row * 2] = rowValues[row];
			newObj[row * 2 + 1] = rowValues[row];
			return newObj;
		}, {});
	}

	function deleteVariable(idx: number) {
		variables.splice(idx, 1);
		variables = variables;

		rowValues = Object.keys(rowValues).reduce((newObj: { [key: number]: boolean }, idx) => {
			const row = parseInt(idx);
			newObj[Math.floor(row / 2)] ||= rowValues[row];
			return newObj;
		}, {});
	}
</script>

<h2>Truth Table to Function</h2>

<p>Click on the cells of column of <span class="math">A</span> to change the truth value.</p>

<button
	type="button"
	id="add-variable-btn"
	title="Add a Variable to the Function"
	disabled={variables.length >= 6}
	on:click={addVariable}
>
	Add Variable
</button>
<small>Up to 6 variables are accepted</small>

<div class="horizontal-overflow-container">
	<table>
		<thead>
			<tr>
				{#each variables as v, variableIdx}
					<th scope="col">
						<input
							type="text"
							name="variableName[{variableIdx}]"
							id="variable-name-{variableIdx}"
							bind:value={v}
							size="1"
							on:focus={(e) => {
								e.currentTarget.setSelectionRange(0, e.currentTarget.value.length);
							}}
						/>
					</th>
				{/each}
				<th scope="col" id="input-col-header"> A({variables.map((n) => n || '?').join(', ')}) </th>
			</tr>
		</thead>
		<tbody>
			{#each Array(Math.pow(2, variables.length)) as _, value}
				<tr>
					{#each Array(variables.length) as _, variableIdx}
						<td class:active={(value >> (variables.length - variableIdx - 1)) % 2}>
							{(value >> (variables.length - variableIdx - 1)) % 2 ? 'T' : 'F'}
						</td>
					{/each}
					<td
						class="input-cell"
						role="checkbox"
						aria-checked={rowValues[value] ? 'true' : 'false'}
						class:active={rowValues[value]}
						on:click={() => {
							rowValues[value] = !(rowValues[value] ?? false);
						}}
						title="Switch Truth Value"
					>
						= {rowValues[value] ? 'T' : 'F'}
					</td>
				</tr>
			{/each}
		</tbody>
		<tfoot>
			<tr>
				{#each variables as v, variableIdx}
					<th scope="col">
						<button
							type="button"
							title="Delete Variable: {v}"
							on:click={() => {
								deleteVariable(variableIdx);
							}}
						>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								fill="none"
								viewBox="0 0 24 24"
								stroke-width="1.5"
								stroke="currentColor"
								width="16"
							>
								<path stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12" />
							</svg>
						</button>
					</th>
				{/each}
				{#if variables.length}
					<th scope="col"></th>
				{/if}
			</tr>
		</tfoot>
	</table>
</div>

<button
	type="button"
	on:click={generateFunction}
	disabled={variables.some((n) => !n) || repeatedNamesExists}
>
	Generate Function
</button>
{#if repeatedNamesExists}
	<small>Repeated variables names</small>
{/if}

{#if functionText}
	<div class="preview">
		<h3>Output</h3>

		<div class="horizontal-overflow-container">
			<p>
				{toPreview(functionText)}
			</p>
		</div>
	</div>
{/if}

<style>
	.input-cell {
		cursor: pointer;
	}

	#input-col-header {
		background-color: var(--colour-2);
	}
	.math {
		font-family: 'Noto Sans Math', sans-serif;
		font-weight: bold;
	}

	thead th input {
		background-color: transparent;
		color: inherit;
		font-family: inherit;
		font-weight: inherit;
		font-size: inherit;
		border-style: solid;
		border-width: 0;
		border-bottom: 1px solid var(--colour-4);
		line-height: 1;
		margin: 0.1em 0;
		text-align: center;
	}

	tfoot button {
		display: grid;
		place-items: center;
	}
	#add-variable-btn {
		margin-bottom: 1rem;
	}

	button + small {
		margin-left: 0.5rem;
	}
</style>
