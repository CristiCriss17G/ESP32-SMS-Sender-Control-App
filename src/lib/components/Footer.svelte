<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { footerState } from '$lib/stores/footer.svelte';
	import { debug } from '@tauri-apps/plugin-log';

	$effect(() => {
		debug(`footerState  B:${footerState.backActive}, N:${footerState.nextActive}`);
	});

	const goToDeviceControl = () => {
		goto(resolve('/device'));
	};
	const goToScan = () => {
		goto(resolve('/'));
	};
</script>

<footer class="grid grid-cols-3 p-4">
	<div class="flex items-center justify-center p-1">
		<button
			class="variant-filled-secondary btn w-full md:w-1/2 lg:w-1/3"
			disabled={!footerState.backActive}
			onclick={goToScan}>Back</button
		>
	</div>
	<div class="flex items-center justify-center p-1">
		<p class="text-center text-sm text-gray-500">© 2025 ESP32 SMS Sender Control App</p>
	</div>
	<div class="flex items-center justify-center p-1">
		<button
			class="variant-filled-primary btn w-full md:w-1/2 lg:w-1/3"
			disabled={!footerState.nextActive}
			onclick={goToDeviceControl}>Next</button
		>
	</div>
</footer>
