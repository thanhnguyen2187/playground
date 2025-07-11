<script lang="ts">
import "../app.css";
import { NuqsAdapter } from "nuqs-svelte/adapters/svelte-kit";

let { children } = $props();
</script>

<NuqsAdapter>
    {@render children()}
</NuqsAdapter>
