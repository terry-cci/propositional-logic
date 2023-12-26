<script lang="ts">
	import { OPERATORS, type Operator, toPreview } from '$lib/toPreview';
	import { createEventDispatcher } from 'svelte';

	function insertOperator(operator: Operator) {
		const startPosition = functionTextarea.selectionStart;
		const endPosition = functionTextarea.selectionEnd;

		functionTextarea.value =
			functionTextarea.value.substring(0, startPosition) +
			operator.inputText +
			functionTextarea.value.substring(endPosition);

		functionTextarea.focus();
		functionTextarea.selectionEnd = startPosition + 1;

		value = functionTextarea.value;
	}

	let functionTextarea: HTMLTextAreaElement;
	export let value = '';

	const dispatch = createEventDispatcher();
</script>

<div>
	<label for="function-textarea">Input Propositional Function:</label>
	<textarea
		name="function"
		id="function-textarea"
		rows="2"
		placeholder="Input the function to be evaluated here..."
		bind:this={functionTextarea}
		bind:value
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
				value = '';
				dispatch('clear');
			}}
		>
			Clear
		</button>
	</div>
</div>

<div class="preview">
	<h3>Function Preview</h3>

	<div class="horizontal-overflow-container">
		<p>
			{#if value}
				{toPreview(value)}
			{:else}
				<small>(Waiting for input...)</small>
			{/if}
		</p>
	</div>
</div>

<style>
	textarea {
		font-family: 'Noto Sans Mono', monospace;
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
</style>
