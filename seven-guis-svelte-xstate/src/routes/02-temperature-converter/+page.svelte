<script lang="ts">
  import { useMachine } from '@xstate/svelte'
  import { temperatureConverterMachine } from './machine'

  const { snapshot, send } = useMachine(temperatureConverterMachine)

  function sendValueCelsius(e: Event) {
    send({ type: 'valueCelsius.set', valueCelsius: Number((e.target as HTMLInputElement).value) })
  }
  function sendValueFahrenheit(e: Event) {
    send({ type: 'valueFahrenheit.set', valueFahrenheit: Number((e.target as HTMLInputElement).value) })
  }
</script>

<div>
  <input
    value={$snapshot.context.valueCelsius}
    on:keyup={sendValueCelsius}
  />
  <span>Celsius</span>
  <input
    value={$snapshot.context.valueFahrenheit}
    on:keyup={sendValueFahrenheit}
  />
  <span>Fahrenheit</span>
</div>

<a href="/">Back</a>
