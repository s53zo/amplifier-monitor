import { writable } from 'svelte/store';

// Define the structure for amplifier data
export interface AmplifierData {
    power: number | string | null; // Allow string initially from backend
    temp: number | string | null;
    swr: number | string | null;
    current: number | string | null;
}

// Define the structure for a notification message
export interface NotificationMessage {
    id: string;
    title: string;
    message: string;
    type: 'info' | 'warning' | 'error' | 'success' | string; // Allow specific types + general string
    duration?: number; // Optional duration in seconds
}

// Store for amplifier data (Map: amplifierName -> AmplifierData)
export const amplifierDataStore = writable<Map<string, AmplifierData>>(new Map());

// Store for active notifications
export const notificationsStore = writable<NotificationMessage[]>([]);

// Store for MQTT connection status (optional, can be added later)
// export const mqttStatusStore = writable<'connecting' | 'connected' | 'disconnected' | 'error'>('disconnected');
