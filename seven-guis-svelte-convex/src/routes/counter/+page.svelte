<script lang="ts">
import { useConvexClient, useQuery } from "convex-svelte";
import { api } from "../../convex/_generated/api";

const query = useQuery(api.counter.get, {});
const client = useConvexClient();

function increase() {
  client.mutation(api.counter.increase, {}).then();
}
</script>

<span>
{#if query.isLoading}
  Loading...
{:else if query.error}
  failed to load: {query.error.toString()}
{:else}
  {query.data}
{/if}
</span>

<br/>

<button onclick={increase}>Increase</button>