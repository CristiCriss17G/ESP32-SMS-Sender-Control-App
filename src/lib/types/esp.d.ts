import { settingsSendSchema } from '$lib/utils/btle';
import { z } from 'zod';

export interface SettingsRead {
	[key: string]: {
		[key: string]: boolean | number | string;
	};
	settings: SettingsSend;
	wifi: WifiStatus;
}

export type SettingsSend = z.infer<typeof settingsSendSchema>;

export interface WifiStatus {
	connected: boolean;
	ipAddress: string;
}
