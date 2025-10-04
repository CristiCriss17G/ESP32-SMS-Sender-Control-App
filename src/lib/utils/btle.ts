import type { BleDevice } from '@mnlphlp/plugin-blec';

import { invoke } from '@tauri-apps/api/core';
import { z } from 'zod';

export async function getSelectedDevice() {
	return await invoke<BleDevice>('get_selected_device');
}

export async function setSelectedDevice(device: BleDevice | null) {
	return await invoke('set_selected_device', { device });
}

export const settingsSendSchema = z
	.object({
		deviceName: z.string().min(4).max(25).nullish(),
		password: z.string().nullish(),
		ssid: z.string().nullish()
	})
	.refine(
		(data) => {
			// We want to ensure that wifiSsid and wifiPassword
			// are both defined or both undefined:
			const { password, ssid } = data;
			return !!ssid === !!password;
		},
		{
			message: 'If wifiSsid is defined, wifiPassword must also be defined, and vice versa.',
			path: ['ssid']
		}
	);
