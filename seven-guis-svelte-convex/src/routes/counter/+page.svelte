<script lang="ts">
import { useQuery } from "convex-svelte";
import { api } from "../../convex/_generated/api";

const counter = useQuery(api.counter.get, {});
</script>
<div>{counter.data}</div>
<button>Increase</button>