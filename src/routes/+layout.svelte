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
			</ul>
		</nav>
		<footer><p>2023, Terry C.C.I.</p></footer>
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

	#side-bar h1 a {
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
</style>
