<script lang="ts">
import { useConvexClient, useQuery } from "convex-svelte";
import { api } from "../../convex/_generated/api";

const query = useQuery(api.counter.get, {});
const client = useConvexClient();

function increase() {
  client.mutation(api.counter.increase, {}).then();
}
</script>

{#if query.isLoading}
  <span>Loading...</span>
{:else if query.error}
  <span>Failed to load: {query.error.toString()}</span>
{:else}
  <input class="input w-20" type="number" value={query.data} />
{/if}

<button class="btn" onclick={increase}>Increase</button>