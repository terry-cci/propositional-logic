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

	async function generateTable() {
		const postfix = await invoke<string[]>('parse_function_text', { text: functionText });
		const tableRows = await invoke('evaluate_truth_table', { postfixExpression: postfix.join('') });
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
</form>

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
</style>
