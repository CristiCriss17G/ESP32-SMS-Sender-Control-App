<script lang="ts">
	interface Props {
		additionalButtons?: import('svelte').Snippet;
		additionalClasses?: string[];
		additionalControls?: import('svelte').Snippet;
		buttons?: boolean;
		classicButtons?: boolean;
		deleteAction?: (() => Promise<void>) | undefined;
		editAction?: () => Promise<void>;
		hrClasses?: string[];
		title: string;
	}

	let {
		additionalButtons,
		additionalClasses = [],
		additionalControls,
		buttons = false,
		classicButtons = true,
		deleteAction = undefined,
		editAction = async () => {
			console.warn('No edit action provided');
		},
		hrClasses = ['!border-t-4', 'my-5'],
		title
	}: Props = $props();
</script>

<div
	class={[
		'flex',
		'flex-col',
		'md:flex-row',
		'gap-4',
		'justify-between',
		'items-center',
		additionalClasses
	]}
>
	<h2 class="h2">{title}</h2>
	<div class="flex flex-col items-center justify-between gap-4 md:flex-row">
		{#if buttons}
			<div class="btn-group preset-filled">
				{#if classicButtons}
					<button onclick={editAction}>Edit</button>
					{#if deleteAction}
						<button onclick={deleteAction}>Delete</button>
					{/if}
				{/if}
				{@render additionalButtons?.()}
			</div>
		{/if}
		{@render additionalControls?.()}
	</div>
</div>

<hr class={hrClasses} />
