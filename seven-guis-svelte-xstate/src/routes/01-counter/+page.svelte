<script lang="ts">
  import { counterMachine } from './machine'
  import { useMachine } from '@xstate/svelte'

  const { snapshot, send } = useMachine(counterMachine)
</script>

<input disabled type="number" value={$snapshot.context.count} />
<button on:click={() => send({type: 'counter.increase'})}>Increase</button> <br/>

<a href="/">Back</a>
