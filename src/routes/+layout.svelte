<script lang="ts">
	import '../app.css';
	import { onNavigate } from '$app/navigation';
	import { resolve } from '$app/paths';
	import logo from '$lib/assets/app-icon.png?enhanced';
	import favicon from '$lib/assets/favicon.ico';
	import Footer from '$lib/components/Footer.svelte';
	import { toaster } from '$lib/stores/toaster-svelte';
	import { AppBar } from '@skeletonlabs/skeleton-svelte';
	import { Toaster } from '@skeletonlabs/skeleton-svelte';

	let { children } = $props();

	onNavigate((navigation) => {
		if (!document.startViewTransition) return;

		return new Promise((resolve) => {
			document.startViewTransition(async () => {
				resolve();
				await navigation.complete;
			});
		});
	});
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
	<title>ESP32 SMS Sender Control App</title>
</svelte:head>

<Toaster {toaster}></Toaster>

<!-- App Shell -->
<div class="grid h-screen grid-rows-[auto_1fr]">
	<!-- Header -->
	<header class="sticky top-0 z-10 backdrop-blur-sm">
		<!-- App Bar -->
		<AppBar>
			{#snippet lead()}
				<a href={resolve('/')} class="mt-4 flex items-center space-x-2 md:mt-0">
					<enhanced:img src={logo} alt="App Logo" class="logo" />
					<h1 class="text-base font-bold uppercase md:text-xl">ESP32 SMS Sender Control App</h1>
				</a>
			{/snippet}
		</AppBar>
	</header>
	<!-- Grid Columns -->
	<div class="grid h-full grid-cols-1 overflow-hidden">
		<!-- Left Sidebar. -->
		<!-- <aside class="overflow-x-hidden overflow-y-auto w-auto">
			<Navigation />
		</aside> -->
		<!-- Page Route Content -->
		<div class="overflow-x-hidden overflow-y-auto">
			<main class="container mx-auto p-10">
				{@render children?.()}
			</main>
		</div>
	</div>
	<Footer />
</div>

<style>
	@keyframes fade-in {
		from {
			opacity: 0;
		}
	}

	@keyframes fade-out {
		to {
			opacity: 0;
		}
	}

	@keyframes slide-from-right {
		from {
			transform: translateX(30px);
		}
	}

	@keyframes slide-to-left {
		to {
			transform: translateX(-30px);
		}
	}

	:root::view-transition-old(root) {
		animation:
			90ms cubic-bezier(0.4, 0, 1, 1) both fade-out,
			300ms cubic-bezier(0.4, 0, 0.2, 1) both slide-to-left;
	}

	:root::view-transition-new(root) {
		animation:
			210ms cubic-bezier(0, 0, 0.2, 1) 90ms both fade-in,
			300ms cubic-bezier(0.4, 0, 0.2, 1) both slide-from-right;
	}

	header {
		view-transition-name: header;
	}

	.logo {
		height: 2.5rem;
		width: 2.5rem;
	}
</style>
