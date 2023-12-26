<script lang="ts">
	let errorDialog: HTMLDialogElement;

	export function showModal() {
		errorDialog.showModal();
	}

	export let errorMsg: string | null;
</script>

<dialog bind:this={errorDialog}>
	<h1>An Error has Occured</h1>

	<p>{errorMsg}</p>

	<form>
		<button type="submit" formmethod="dialog">Close</button>
	</form>
</dialog>
