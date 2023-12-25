<script lang="ts">
	import FunctionInput from '$lib/components/FunctionInput.svelte';

	let functionText = '';
</script>

<h2>Normalise Function</h2>
<form>
	<FunctionInput bind:value={functionText} />
</form>
