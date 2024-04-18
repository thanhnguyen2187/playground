<script lang="ts">
  import { flightBookerMachine } from './machine'
  import { useMachine } from '@xstate/svelte'

  const { snapshot, send } = useMachine(flightBookerMachine)

  function handleFlightTypeChange(e: Event) {
    // @ts-ignore
    send({
      type: 'flightType.set',
      value: (e.target as HTMLInputElement).value
    })
  }

  function handleFromDateChange(e: Event) {
    send({
      type: 'fromDate.set',
      value: (e.target as HTMLInputElement).value,
    })
  }

  function handleToDateChange(e: Event) {
    send({
      type: 'toDate.set',
      value: (e.target as HTMLInputElement).value,
    })
  }
</script>

<style>
  .warning {
      background-color: orangered;
  }
</style>

<select on:change={handleFlightTypeChange}>
  <option value="oneway">one-way flight</option>
  <option value="return">return flight</option>
</select> <br/>
<input
  value={$snapshot.context.fromDate}
  on:keyup={handleFromDateChange}
  class:warning={$snapshot.context.fromDateWarning}
/> <br/>
<input
  value={$snapshot.context.toDate}
  on:keyup={handleToDateChange}
  class:warning={$snapshot.context.toDateWarning}
  disabled={!$snapshot.context.toDateEnabled}
/> <br/>
<button
  disabled={!$snapshot.context.bookingEnabled}
>Book</button>

<a href="/">Back</a>
