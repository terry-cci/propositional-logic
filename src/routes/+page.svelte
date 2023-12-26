<article>
	<h2>Propositional Function Evaluator</h2>

	<p>
		This is a desktop application built with Tauri + SvelteKit (Rust + JavaScript), capable of
		automatically evaluating propositional functions and truth tables.
	</p>

	<p>Please click the navigation links on the left for corresponding features.</p>

	<h3>How does this work?</h3>

	<h4>Function to Truth Table</h4>

	<p>
		Functions are first converted into postfix expression, then a expression tree is constructed.
	</p>
	<p>
		Iterating through all possible truth values of variables, the truth table is generated. Partial
		expressions are also stored throughout the evaluation.
	</p>

	<h4>Truth Table to Function</h4>

	<p>
		A normal form is chosen based on whether there are more T's or F's. Then the function is sent to
		normalisation.
	</p>

	<h4>Normalisation Function</h4>

	<p>
		Iterating through the rows in truth table, variables are determined whether to be negated or
		not. Operations between parts are dependent to whether DNF or CNF is chosen.
	</p>

	<footer>
		Terry C.C.I.
		<br />
		Dec, 2023
	</footer>
</article>

<style>
	h3 {
		border-bottom: 0.5px solid var(--colour-4);
		padding-bottom: 0.2em;
	}
	h3 + h4 {
		margin-top: 0.5em;
	}
	h4 {
		margin-top: 2em;
		margin-bottom: 0.5em;
	}
	h4::before {
		content: '· · · ';
	}
	h4::after {
		content: ' · · ·';
	}
	h4 + p {
		margin-top: 0.5em;
	}

	footer {
		text-align: center;
		margin-top: 4rem;
		font-size: 0.8em;
		color: var(--colour-4);
		padding-bottom: 4rem;
	}
</style>
