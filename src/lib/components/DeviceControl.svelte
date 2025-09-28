<script lang="ts">
	import type { SettingsRead, SettingsSend } from '$lib/types/esp';

	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import {
		PUBLIC_NOTIFY_CHARACTERISTIC_UUID,
		PUBLIC_READ_WRITE_CHARACTERISTIC_UUID
	} from '$env/static/public';
	import { footerState } from '$lib/stores/footer.svelte';
	import { toaster } from '$lib/stores/toaster-svelte';
	import { getSelectedDevice } from '$lib/utils/btle';
	import { settingsSendSchema } from '$lib/utils/btle';
	import { LoaderCircle } from '@lucide/svelte';
	import {
		connect,
		disconnect,
		getConnectionUpdates,
		readString,
		sendString,
		subscribeString
	} from '@mnlphlp/plugin-blec';
	import { Modal } from '@skeletonlabs/skeleton-svelte';
	import { debug } from '@tauri-apps/plugin-log';
	import { onMount } from 'svelte';

	import ItemHeaderRow from './ItemHeaderRow.svelte';

	let connected: boolean = $state(false);
	let wifiData: SettingsRead | undefined = $state();
	let formData: SettingsSend = $state({});
	let needsRestart: boolean = $state(false);
	let readyForRestart: boolean = $state(false);

	async function readSettings() {
		wifiData = JSON.parse(await readString(PUBLIC_READ_WRITE_CHARACTERISTIC_UUID)) as SettingsRead;
		debug(`Read settings: ${JSON.stringify(wifiData)}`);
	}

	async function handleSubmit(e: Event) {
		e.preventDefault();
		const validFields = settingsSendSchema.safeParse(formData);
		debug(`Valid fields: ${JSON.stringify(validFields)}`);
		if (!validFields.success) {
			let duration = 5000;
			validFields.error.issues.forEach((issue) => {
				toaster.error({
					duration,
					title: `${issue.path}: ${issue.message}`
				});
				duration += 500;
			});
			return;
		}

		// create a new object with only the keys that defined and not empty
		const data = Object.fromEntries(
			Object.entries(formData).filter(([, v]) => v !== undefined && v !== null && v !== '')
		);
		// if the wifi credentials, servr endpoint or api key are changed, the device needs to restart
		needsRestart = data.ssid !== null || data.password !== null;
		debug(`Sending data: ${JSON.stringify(data)}`);
		await sendString(PUBLIC_READ_WRITE_CHARACTERISTIC_UUID, JSON.stringify(data));
	}

	let modalState = $state(false);

	function modalClose() {
		modalState = false;
	}

	async function modalConfirm() {
		modalState = false;
		readyForRestart = false;
		needsRestart = false;
		sendString(PUBLIC_READ_WRITE_CHARACTERISTIC_UUID, JSON.stringify({ restart: true }));
		debug('Restarting device');
		await goto(resolve('/'));
	}

	function restartModal() {
		modalState = true;
	}

	onMount(() => {
		footerState.backActive = true;
		footerState.nextActive = false;

		(async () => {
			const selectedDevice = await getSelectedDevice();
			await connect(selectedDevice.address, null);
			await getConnectionUpdates((c) => {
				connected = c;
			});
			await readSettings();
			formData = { ...wifiData?.settings, password: '', ssid: '' };
			await subscribeString(PUBLIC_NOTIFY_CHARACTERISTIC_UUID, (data) => {
				debug(`Status: ${data}`);
				toaster.info({
					duration: 5000,
					title: `Status: ${data}`
				});
				readyForRestart = readyForRestart ? readyForRestart : data.includes('NR');
				console.log(readyForRestart);
				console.log(data);
				if (needsRestart && readyForRestart) {
					restartModal();
				}
			});
		})();

		return () => {
			disconnect();
			footerState.backActive = false;
			footerState.nextActive = false;
		};
	});
</script>

<Modal
	open={modalState}
	onOpenChange={(e) => (modalState = e.open)}
	triggerBase="btn preset-tonal"
	contentBase="card bg-surface-100-900 p-4 space-y-4 shadow-xl max-w-screen-sm"
	backdropClasses="backdrop-blur-sm"
>
	{#snippet content()}
		<header class="flex justify-between">
			<h2 class="h2">Restart Device</h2>
		</header>
		<article>
			<p class="opacity-60">
				The device needs to restart to apply the changes. Do you want to restart now?
			</p>
		</article>
		<footer class="flex justify-end gap-4">
			<button type="button" class="btn preset-tonal" onclick={modalClose}>Cancel</button>
			<button type="button" class="btn preset-filled" onclick={modalConfirm}>Confirm</button>
		</footer>
	{/snippet}
</Modal>

<ItemHeaderRow title="Control device" buttons={false} />

<h3 class="my-2 text-center h3">
	{#if connected}
		Connected
	{:else}
		Connecting...
	{/if}
</h3>

<div class="card p-4">
	<div class="flex items-center justify-between">
		<p class="mb-0">WiFi Data:</p>
		<button class="variant-outline-primary btn" onclick={readSettings}> Read WiFi Data </button>
	</div>
	<div class="table-container mt-2">
		<table class="table-compact table">
			<thead>
				<tr>
					<th>Connection status</th>
					<th>Wifi Network</th>
					<th>IP Address</th>
				</tr>
			</thead>
			<tbody>
				{#if !wifiData}
					<tr>
						<td colspan="3" class="text-center">
							<LoaderCircle class="me-3 inline h-4 w-4 animate-spin text-white" />
						</td>
					</tr>
				{:else}
					<tr>
						<td>{wifiData.wifiStatus.connected}</td>
						<td>{wifiData.settings.ssid}</td>
						<td>{wifiData.wifiStatus.ipAddress}</td>
					</tr>
				{/if}
			</tbody>
		</table>
	</div>
</div>

<form class="text-token my-4 w-full space-y-4 card p-4" action="">
	<label for="deviceName" class="label">
		<span>Device name</span>
		<input type="text" class="input" name="deviceName" bind:value={formData.deviceName} />
	</label>
	<label for="ssid" class="label">
		<span>WiFi SSID</span>
		<input
			type="text"
			class="input"
			name="ssid"
			bind:value={formData.ssid}
			placeholder={wifiData?.settings.ssid}
		/>
	</label>
	<label for="password" class="label">
		<span>WiFi Password</span>
		<input
			type="password"
			class="input"
			name="password"
			bind:value={formData.password}
			placeholder={wifiData?.settings.password}
		/>
	</label>
	<div class="mt-6 flex items-center justify-end gap-x-6">
		<button class="variant-filled-secondary btn" type="reset"> Reset </button>
		<button class="variant-filled-primary btn" onclick={handleSubmit} type="submit">
			Update
		</button>
	</div>
</form>
