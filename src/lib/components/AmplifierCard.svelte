<script lang="ts">
	import type { AmplifierData } from '$lib/stores';

	// Props: amplifier name and its data object
	export let name: string;
	export let data: AmplifierData | undefined; // Data might be undefined initially

	// Helper to format values, handling null/undefined
	function formatValue(value: number | string | null | undefined, unit: string = ''): string {
		if (value === null || value === undefined) {
			return '--';
		}
        // If it's a number, format it nicely (e.g., fixed decimals)
        if (typeof value === 'number') {
             // Basic formatting, can be customized
            if (unit === '°C' || unit === 'A' || unit === '') { // Temp, Current, SWR
                 return value.toFixed(1) + unit;
            } else { // Power (W)
                return value.toFixed(0) + unit;
            }
        }
		return String(value) + unit; // Return string value directly if not a number
	}
</script>

<div class="amplifier-card">
	<h3>{name}</h3>
	<div class="metrics">
		<div>Power: {formatValue(data?.power, ' W')}</div>
		<div>Temp: {formatValue(data?.temp, ' °C')}</div>
		<div>SWR: {formatValue(data?.swr)}</div>
		<div>Current: {formatValue(data?.current, ' A')}</div>
	</div>
</div>

<style>
	.amplifier-card {
		border: 1px solid #ccc;
		padding: 1rem;
		margin: 0.5rem;
		border-radius: 5px;
		min-width: 150px; /* Ensure some minimum width */
        background-color: #f9f9f9;
	}
	h3 {
		margin-top: 0;
        margin-bottom: 0.5rem;
        text-align: center;
        font-size: 1.1em;
	}
    .metrics div {
        margin-bottom: 0.25rem;
    }
</style>
