<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { listen } from '@tauri-apps/api/event';
	import { amplifierDataStore, notificationsStore } from '$lib/stores';
	import type { AmplifierData, NotificationMessage } from '$lib/stores';
	import NotificationArea from '$lib/components/NotificationArea.svelte'; // Import the component

	// Define the expected payload structures from the backend
	interface MqttDataEventPayload {
		amplifierName: string;
		metric: keyof AmplifierData; // 'power', 'temp', 'swr', 'current'
		value: string; // Value comes as string from backend
	}

	interface MqttNotificationEventPayload extends NotificationMessage {
		// Backend sends the full NotificationMessage structure
	}

	let unlistenData: (() => void) | null = null;
	let unlistenNotification: (() => void) | null = null;

	onMount(async () => {
		console.log('Layout mounted, setting up listeners...');

		// Listener for amplifier data updates
		unlistenData = await listen<MqttDataEventPayload>('mqtt-data', (event) => {
			console.log('Received mqtt-data:', event.payload);
			const { amplifierName, metric, value } = event.payload;

			amplifierDataStore.update((currentMap) => {
				const ampData = currentMap.get(amplifierName) || { power: null, temp: null, swr: null, current: null };
                // Attempt to parse numeric values, keep as string if parsing fails
                let parsedValue: number | string | null = parseFloat(value);
                if (isNaN(parsedValue)) {
                    parsedValue = value; // Keep original string if not a number
                }
                // Handle null/empty strings explicitly if needed
                if (value === null || value.trim() === "") {
                    parsedValue = null;
                }

				// Update the specific metric
				ampData[metric] = parsedValue;

				currentMap.set(amplifierName, ampData);
				return new Map(currentMap); // Return new Map to trigger reactivity
			});
		});

		// Listener for notifications
		unlistenNotification = await listen<MqttNotificationEventPayload>('mqtt-notification', (event) => {
			console.log('Received mqtt-notification:', event.payload);
			const notification = event.payload;
			notificationsStore.update((currentNotifications) => {
				// Avoid adding duplicate notifications if backend sends rapidly (simple check)
				if (!currentNotifications.some(n => n.id === notification.id)) {
					return [...currentNotifications, notification];
				}
				return currentNotifications;
			});
		});

        console.log('Listeners setup complete.');

		// Clean up listeners when the component is destroyed
		return () => {
            console.log('Layout unmounting, cleaning up listeners...');
			if (unlistenData) unlistenData();
			if (unlistenNotification) unlistenNotification();
            console.log('Listeners cleaned up.');
		};
	});

    // Optional: Implement onDestroy as well for safety, though return from onMount should suffice
    onDestroy(() => {
        console.log('Layout destroyed, ensuring listeners are cleaned up...');
        if (unlistenData) unlistenData();
        if (unlistenNotification) unlistenNotification();
    });

</script>

<!-- Basic App Layout -->
<NotificationArea /> {/* Add the NotificationArea component here */}

<main>
	<!-- Page content is rendered here -->
	<slot />
</main>

<style>
	/* Add global styles if needed */
	main {
		padding: 1rem;
	}
</style>
