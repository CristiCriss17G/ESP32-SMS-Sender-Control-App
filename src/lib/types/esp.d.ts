import { settingsSendSchema } from '$lib/utils/btle';
import { z } from 'zod';

export interface SettingsRead {
	settings: SettingsSend;
	wifiStatus: WifiStatus;
}

export type SettingsSend = z.infer<typeof settingsSendSchema>;

export interface WifiStatus {
	connected: boolean;
	ipAddress: string;
}
