<script lang="ts">
	import type { BleDevice } from '@mnlphlp/plugin-blec';

	import { PUBLIC_SERVICE_UUID } from '$env/static/public';
	import { footerState } from '$lib/stores/footer.svelte';
	import { setSelectedDevice } from '$lib/utils/btle';
	import { LoaderCircle } from '@lucide/svelte';
	import { getScanningUpdates, startScan, stopScan } from '@mnlphlp/plugin-blec';
	import { Segment } from '@skeletonlabs/skeleton-svelte';
	import { debug } from '@tauri-apps/plugin-log';
	import { onMount } from 'svelte';

	import ItemHeaderRow from './ItemHeaderRow.svelte';

	const devices: BleDevice[] = $state([]);
	let scanning: boolean = $state(false);
	const handler = (hDevices: BleDevice[]) => {
		hDevices.forEach((device) => {
			if (
				!devices.find((d) => d.address === device.address) &&
				device.services.includes(PUBLIC_SERVICE_UUID)
			) {
				devices.push(device);
			}
		});
	};
	let selectedDevice: BleDevice | null = $state(null);

	const doScan = async () => {
		devices.length = 0;
		selectedDevice = null;
		await startScan(handler, 10000);
		setTimeout(stopScan, 10000);
	};

	onMount(async () => {
		await doScan();
		await getScanningUpdates((s) => {
			scanning = s;
		});
	});
	$effect(() => {
		debug(`Devices: ${JSON.stringify($state.snapshot(devices))}`);
		debug(`Scanning: ${JSON.stringify($state.snapshot(scanning))}`);
		debug(`Selected Device: ${JSON.stringify($state.snapshot(selectedDevice))}`);
		setSelectedDevice(selectedDevice);
		if (selectedDevice && !scanning) {
			footerState.nextActive = true;
		} else {
			footerState.nextActive = false;
		}
	});
</script>

<ItemHeaderRow title="Scan for SMS Sender Devices" buttons={false} />

<div class="card">
	<section class="py-4">
		<Segment
			classes="w-full"
			name="btle-devices"
			orientation="vertical"
			value={selectedDevice?.address}
			onValueChange={(e) => (selectedDevice = devices.find((d) => d.address === e.value) || null)}
		>
			{#each devices as device, i (device.address)}
				<Segment.Item classes="justify-start" value={device.address}>
					<span>{i + 1}.</span>
					<span class="flex-auto">{device.name} - {device.address}</span>
				</Segment.Item>
			{/each}
		</Segment>
	</section>
	<footer class="card-footer">
		<div
			class={[
				'input-group',
				'input-group-divider',
				scanning ? 'grid-cols-2' : 'grid-cols-1',
				'transition-all',
				'duration-300',
				'ease-in-out'
			]}
		>
			<button
				type="button"
				class={[
					'!justify-center',
					'p-2',
					'disabled:cursor-not-allowed',
					'preset-tonal-primary',
					scanning && ['border-e', 'border-solid', 'border-e-primary-100-900'],
					'gap-1'
				]}
				onclick={doScan}
				disabled={scanning}
			>
				Scan
				{#if scanning}
					<LoaderCircle class="me-3 inline h-4 w-4 animate-spin dark:text-white" />
				{/if}
			</button>
			{#if scanning}
				<button type="button" class="btn-control preset-tonal-secondary" onclick={stopScan}>
					Stop scan
				</button>
			{/if}
		</div>
	</footer>
</div>
