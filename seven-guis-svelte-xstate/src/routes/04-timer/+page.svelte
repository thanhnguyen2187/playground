<script lang="ts">
  import { timerMachine } from './machine'
  import { useMachine } from '@xstate/svelte'

  const { snapshot, send } = useMachine(timerMachine)
  send({type: 'start'})

  function handleDurationChange(e: Event) {
    send({
      type: 'duration.set',
      value: Number((e.target as HTMLInputElement).value),
    })
  }

  function reset() {
    send({type: 'reset'})
  }
</script>

<div>
  <span>Elapsed time</span>
  <progress value="{$snapshot.context.elapsed / $snapshot.context.duration}"></progress> <br/>
  {$snapshot.context.elapsed / 1000} s <br/>
  <input
    type="range"
    value={$snapshot.context.duration}
    min={5_000}
    max={60_000}
    on:change={handleDurationChange}
  />
  <button on:click={reset}>Reset</button>
</div>

<a href="/">Back</a>
