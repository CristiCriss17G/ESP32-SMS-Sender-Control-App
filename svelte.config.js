import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		adapter: adapter({
			// default options are shown. On some platforms
			assets: 'build',
			fallback: undefined,
			// these options are set automatically — see below
			pages: 'build',
			strict: true
		}),
		output: {
			bundleStrategy: 'single'
		}
	},
	// Consult https://svelte.dev/docs/kit/integrations
	// for more information about preprocessors
	preprocess: vitePreprocess()
};

export default config;
