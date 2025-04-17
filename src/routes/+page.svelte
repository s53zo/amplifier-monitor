<script lang="ts">
	import { amplifierDataStore } from '$lib/stores';
	import AmplifierCard from '$lib/components/AmplifierCard.svelte';

	// Get the amplifier names from the Map keys for iteration
	$: amplifierNames = Array.from($amplifierDataStore.keys());

    // TODO: Ideally, the initial amplifier names should be populated
    // based on the app-config.json when the app starts, perhaps
    // by having the backend emit an initial state event or by reading
    // the config on the frontend side if necessary.
    // For now, the store will populate as data arrives.
</script>

<div class="page-content">
	<h1>Amplifier Monitor</h1>

	<div class="card-container">
		{#if amplifierNames.length > 0}
			{#each amplifierNames as name (name)}
				<AmplifierCard {name} data={$amplifierDataStore.get(name)} />
			{:else}
				<p>Waiting for amplifier data...</p>
			{/each}
		{:else}
			 <p>No amplifiers configured or waiting for data...</p>
             <!-- Consider adding a message prompting user to check config if empty after a while -->
		{/if}
	</div>
</div>

<style>
	.page-content {
		/* Styles for the page container if needed */
	}
	h1 {
		text-align: center;
		margin-bottom: 1.5rem;
	}
	.card-container {
		display: flex;
		flex-wrap: wrap; /* Allow cards to wrap to next line */
		justify-content: center; /* Center cards horizontally */
		gap: 1rem; /* Spacing between cards */
	}
    p {
        text-align: center;
        width: 100%;
        color: #666;
    }
</style>
