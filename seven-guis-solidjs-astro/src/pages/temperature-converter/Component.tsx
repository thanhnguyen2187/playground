import { createSignal } from "solid-js";

export default function Component() {
  const [values, setValues] = createSignal({
    celsius: 0,
    fahrenheit: 32,
  });

  function setValueCelsius(value: number) {
    setValues({
      celsius: value,
      fahrenheit: Math.round((value * (9 / 5) + 32) * 100) / 100,
    });
  }

  function setValueFahrenheit(value: number) {
    setValues({
      fahrenheit: value,
      celsius: Math.round((value - 32) * (5 / 9) * 100) / 100,
    });
  }

  function handleValueCelsiusChange(e: Event) {
    const input = e.target as HTMLInputElement;
    const valueCelsius = Number(input.value);
    setValueCelsius(valueCelsius);
  }

  function handleValueFahrenheitChange(e: Event) {
    const input = e.target as HTMLInputElement;
    const valueFahrenheit = Number(input.value);
    setValueFahrenheit(valueFahrenheit);
  }

  return (
    <form class={"flex flex-col gap-2"}>
      <fieldset class={"fieldset"}>
        <legend class={"fieldset-legend"}>Celsius</legend>
        <input
          class={"input"}
          type={"number"}
          on:keyup={handleValueCelsiusChange}
          value={values().celsius}
        />
      </fieldset>
      <span>=</span>
      <fieldset class={"fieldset"}>
        <legend class={"fieldset-legend"}>Fahrenheit</legend>
        <input
          class={"input"}
          type={"number"}
          on:keyup={handleValueFahrenheitChange}
          value={values().fahrenheit}
        />
      </fieldset>
    </form>
  );
}
