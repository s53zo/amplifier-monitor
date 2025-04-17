<script lang="ts">
	import { notificationsStore } from '$lib/stores';
	import NotificationPopup from './NotificationPopup.svelte';
	import { fly } from 'svelte/transition'; // Import a transition

	// No need to explicitly subscribe here, Svelte's reactive '$' syntax handles it in the template
</script>

<div class="notification-area">
	{#each $notificationsStore as notification (notification.id)}
		<div transition:fly={{ y: -20, duration: 300 }}> {/* Add a simple transition */}
			<NotificationPopup {notification} />
		</div>
	{/each}
</div>

<style>
	.notification-area {
		position: fixed; /* Position relative to the viewport */
		top: 1rem;
		right: 1rem;
		z-index: 1000; /* Ensure it's above other content */
		display: flex;
		flex-direction: column;
		align-items: flex-end; /* Align popups to the right */
	}
</style>
