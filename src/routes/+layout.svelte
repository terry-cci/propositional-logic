<script lang="ts">
	import { open } from '@tauri-apps/api/shell';
</script>

<div class="default-layout">
	<div id="side-bar">
		<h1>
			<a href="/">Propositional Function<br />Evaluator</a>
		</h1>
		<nav>
			<ul>
				<li>
					<a href="/f2tt"> Function to Truth&nbsp;Table </a>
				</li>
				<li>
					<a href="/tt2f"> Truth&nbsp;Table to Function </a>
				</li>
				<li>
					<a href="/f2nf"> Normalise Function </a>
				</li>
			</ul>
		</nav>
		<footer>
			<p>
				2023, <a
					href="https://github.com/terry-cci"
					on:click|preventDefault={() => {
						open('https://github.com/terry-cci');
					}}>Terry C.C.I.</a
				>
			</p>
		</footer>
	</div>
	<main>
		<slot />
	</main>
</div>

<style>
	@import url('https://fonts.googleapis.com/css2?family=Noto+Sans+Math&family=Noto+Sans+Mono&family=Noto+Sans:ital,wght@0,400;0,700;1,400;1,700&display=swap');

	:global(:root) {
		--colour-1: #07598c;
		--colour-2: #043959;
		--colour-3: #07668c;
		--colour-4: #5aa3bf;
		--colour-5: #c4e5f2;
	}
	:global(*) {
		box-sizing: border-box;
	}

	:global(body) {
		color: var(--colour-2);
		background-color: var(--colour-5);
		margin: 0;

		font-family: 'Noto Sans', sans-serif;
	}

	.default-layout {
		display: flex;
		height: 100vh;
	}

	#side-bar {
		height: 100%;
		background-color: var(--colour-2);
		color: var(--colour-5);
		flex-shrink: 0;
		padding: 0.5rem 1.5rem;
		max-width: 30vw;

		display: flex;
		flex-direction: column;
		box-shadow: 0px 0px 4px var(--colour-2);
	}
	#side-bar h1 {
		font-size: 1.5rem;
		padding-bottom: 0.5em;
		border-bottom: 1px solid var(--colour-1);
	}

	#side-bar h1 a,
	#side-bar footer a {
		color: inherit;
		text-decoration: none;
	}

	#side-bar nav ul {
		padding-left: 1em;
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	#side-bar nav a {
		color: inherit;
		text-decoration: none;
		border-bottom: 1px solid transparent;
		transition-duration: 100ms;
		transition-property: color border-bottom-color;
	}

	#side-bar nav a:hover {
		color: var(--colour-4);
		border-bottom-color: var(--colour-4);
	}

	#side-bar footer {
		flex-grow: 1;
		display: flex;
		flex-direction: column;
		justify-content: flex-end;
		color: var(--colour-4);
		font-size: small;
		text-align: center;
	}

	main {
		flex-grow: 1;
		padding: 1rem 2.5rem;
		overflow-y: auto;
	}

	:global(main h2) {
		border-bottom: 0.5px solid var(--colour-4);
		padding-bottom: 0.3em;
	}

	:global(form) {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	:global(form > div) {
		display: flex;
		flex-direction: column;
		gap: 0.25em;
	}

	:global(form > button[type='submit']) {
		font-weight: bold;
		font-size: 1rem;
		margin-top: 1em;
		width: max-content;
		margin-left: auto;
		margin-right: auto;
		padding: 0.5em 3em;
		background-color: var(--colour-3);
	}
	:global(form > button[type='submit']):hover {
		background-color: var(--colour-2);
	}
	:global(form > button[type='submit']):active {
		background-color: var(--colour-3);
	}
	:global(form > button[type='submit']):disabled {
		background-color: var(--colour-3);
	}

	:global(form > button[type='submit'] + small) {
		display: block;
		text-align: center;
		margin-top: -0.5rem;
	}

	:global(textarea) {
		resize: vertical;
		background-color: #e7f9ff;
		padding: 0.5em;
		border-radius: 0.25rem;
	}

	:global(button) {
		cursor: pointer;
		border-radius: 0.25rem;
		background-color: var(--colour-2);
		color: var(--colour-5);
		min-width: 3em;
		padding: 0.2em 1em;
		transition-duration: 100ms;
		transition-property: background-color;
		font-family: 'Noto Sans', sans-serif;
	}

	:global(button:hover) {
		background-color: var(--colour-1);
	}

	:global(button:active) {
		background-color: var(--colour-3);
	}

	:global(button:disabled) {
		opacity: 0.5;
		background-color: var(--colour-2);
		cursor: not-allowed;
	}

	:global(table) {
		width: max-content;
		border-collapse: collapse;
	}

	:global(table td),
	:global(table th) {
		border: 1px solid var(--colour-4);
		padding: 0.25em 1em;
	}

	:global(table thead th) {
		background-color: var(--colour-3);
		color: var(--colour-5);
		font-family: 'Noto Sans Math', sans-serif;
	}

	:global(table td.active) {
		background-color: #91d6f1;
		font-weight: bold;
	}

	:global(.horizontal-overflow-container) {
		overflow-x: auto;
		margin: 0 -2.5rem;
		padding: 0 2.5rem;
		padding-bottom: 1rem;
	}

	:global(.preview p) {
		margin: 0;
		text-align: center;
		font-family: 'Noto Sans Math', sans-serif;
		font-size: 1.5rem;
	}

	:global(.preview small) {
		font-size: small;
	}

	:global(.input-text) {
		font-family: 'Noto Sans Mono', monospace;
		text-align: center;
		font-size: smaller;
		color: var(--colour-4);
	}

	:global(dialog h1) {
		font-size: 1.3rem;
	}

	:global(dialog) {
		border-radius: 0.5rem;
		box-shadow: 0 0 5px var(--colour-3);
		padding: 2rem 2.5rem;
	}

	:global(dialog::backdrop) {
		background-color: rgba(4, 57, 89, 0.8);
	}

	:global(.send-btn) {
		display: block;
		margin: 0.5rem auto;
	}
</style>
