<script lang="ts">
	import { onDestroy } from 'svelte';
	import { notificationsStore } from '$lib/stores';
	import type { NotificationMessage } from '$lib/stores';

	export let notification: NotificationMessage;

	const DEFAULT_DURATION_MS = 5000; // 5 seconds

	let timeoutId: ReturnType<typeof setTimeout> | null = null;

	function dismiss() {
		if (timeoutId) clearTimeout(timeoutId);
		notificationsStore.update((n) => n.filter((item) => item.id !== notification.id));
	}

	// Auto-dismiss after duration
	const durationMs = notification.duration ? notification.duration * 1000 : DEFAULT_DURATION_MS;
	if (durationMs > 0) { // Allow duration 0 or negative for persistent notifications
		timeoutId = setTimeout(dismiss, durationMs);
	}

	// Ensure timeout is cleared if component is destroyed early
	onDestroy(() => {
		if (timeoutId) clearTimeout(timeoutId);
	});

    // Determine background color based on type
    $: bgColor = (() => {
        switch (notification.type) {
            case 'error': return '#f8d7da'; // Light red
            case 'warning': return '#fff3cd'; // Light yellow
            case 'success': return '#d1e7dd'; // Light green
            case 'info': return '#cff4fc'; // Light blue
            default: return '#e2e3e5'; // Default light gray
        }
    })();

     $: textColor = (() => {
        switch (notification.type) {
            case 'error': return '#842029'; // Dark red
            case 'warning': return '#664d03'; // Dark yellow/brown
            case 'success': return '#0f5132'; // Dark green
            case 'info': return '#055160'; // Dark blue
            default: return '#41464b'; // Default dark gray
        }
    })();

</script>

<div class="notification-popup" style="background-color: {bgColor}; color: {textColor}; border-left: 5px solid {textColor};">
	<div class="content">
		<strong>{notification.title || notification.type.toUpperCase()}</strong>
		<p>{notification.message}</p>
	</div>
	<button class="dismiss-btn" on:click={dismiss}>&times;</button>
</div>

<style>
	.notification-popup {
		position: relative; /* Needed for absolute positioning of button */
		padding: 0.8rem 1.5rem 0.8rem 1rem; /* More padding on right for button */
		margin-bottom: 0.5rem;
		border-radius: 4px;
		box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
        min-width: 250px;
        max-width: 400px;
	}
    .content {
        flex-grow: 1;
    }
    .content strong {
        font-weight: bold;
    }
    .content p {
        margin: 0.3rem 0 0 0;
        font-size: 0.9em;
        word-wrap: break-word; /* Prevent long messages overflowing */
    }

	.dismiss-btn {
		position: absolute;
        top: 2px;
        right: 5px;
		background: none;
		border: none;
		font-size: 1.5rem;
		line-height: 1;
		cursor: pointer;
		color: inherit; /* Use text color */
        opacity: 0.7;
        padding: 0.2rem;
	}
    .dismiss-btn:hover {
        opacity: 1;
    }
</style>
