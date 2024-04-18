<script lang="ts">

  type Unit = 'celsius' | 'fahrenheit'
  type Temperature = {
    value: number,
    unit: Unit,
  }

  let currentTemperature: Temperature = {
    value: 0,
    unit: 'celsius',
  }

  function toCelsius(temperature: Temperature): number {
    if (temperature.unit == 'celsius') {
      return temperature.value
    } else if (temperature.unit == 'fahrenheit') {
      return (temperature.value - 32) * 5 / 9
    } else {
      throw 'unreachable code: invalid temperature unit ' + temperature.unit
    }
  }

  function toFahrenheit(temperature: Temperature): number {
    if (temperature.unit == 'celsius') {
      return temperature.value * 9 / 5 - 32
    } else if (temperature.unit == 'fahrenheit') {
      return temperature.value
    } else {
      throw 'unreachable code: invalid temperature unit ' + temperature.unit
    }
  }

  function setCurrentValueCelsius(event: InputEvent) {
    currentTemperature.value = Number.parseFloat((event.target as HTMLInputElement).value)
    currentTemperature.unit = 'celsius'
  }

  function setCurrentValueFahrenheit(event: InputEvent) {
    currentTemperature.value = Number.parseFloat((event.target as HTMLInputElement).value)
    currentTemperature.unit = 'fahrenheit'
  }

</script>

<div>
  <input
    type="number"
    value={+toCelsius(currentTemperature).toFixed(2)}
    on:input={setCurrentValueCelsius}
  />
  <span>Celsius</span> =
  <input
    type="number"
    value={+toFahrenheit(currentTemperature).toFixed(2)}
    on:input={setCurrentValueFahrenheit}
  />
  <span>Fahrenheit</span>
</div>
