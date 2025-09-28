export function humanReadableDateTime(date: string): string {
	const localeInfo = Intl.DateTimeFormat().resolvedOptions();
	const options: Intl.DateTimeFormatOptions = {
		hour12: localeInfo.hour12 === true,
		timeZone: localeInfo.timeZone
	};
	return new Date(date).toLocaleString(localeInfo.locale, options);
}
