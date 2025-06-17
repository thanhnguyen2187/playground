import { Show, createMemo } from "solid-js";
import { createStore } from "solid-js/store";
type FlightType = "one-way-flight" | "return-flight";

function validateDate(value: string): {
  isValid: boolean;
  message: string | undefined;
} {
  const isMatched = value.match(/\d{2}\.\d{2}\.\d{4}/);
  if (!isMatched) {
    return {
      isValid: false,
      message:
        "Invalid date format; expected dd.mm.yyyy " +
        "(precisely 2 numbers for day, 2 numbers for month and 4 numbers for year)",
    };
  }

  const parts = value.split(".");
  const day = Number(parts[0]);
  const month = Number(parts[1]);
  const year = Number(parts[2]);

  if (day < 1 || day > 31) {
    return {
      isValid: false,
      message: "Invalid day input; day should be >= 1 and <= 31",
    };
  }
  if (month < 1 || month > 12) {
    return {
      isValid: false,
      message: "Invalid month input; month should be >= 1 and <= 31",
    };
  }
  if (year < 2020 || year > 2100) {
    return {
      isValid: false,
      message: "Invalid year input; year should be >= 2020 and <= 2100",
    };
  }

  return {
    isValid: true,
    message: undefined,
  };
}

function validateFromToDate(fromDate: string, toDate: string) {
  // if (!validateDate(fromDate).isValid || !validateDate(toDate)) {
  //   return {
  //     isValid: false,
  //     message: "Unreachable code!",
  //   };
  // }
  if (fromDate > toDate) {
    return {
      isValid: false,
      message: "From date must be before to date",
    };
  }

  return {
    isValid: true,
    message: undefined,
  };
}

export default function Component() {
  const [formStore, setFormStore] = createStore<{
    flightType: FlightType;
    fromDate: string;
    toDate: string;
    errors: Partial<Record<string, string>>;
  }>({
    flightType: "one-way-flight",
    fromDate: "",
    toDate: "",
    errors: {},
  });
  let fromDateRef!: HTMLInputElement;
  let toDateRef!: HTMLInputElement;

  const fromDateValidation = createMemo(() => {
    return validateDate(formStore.fromDate);
  });
  const toDateValidation = createMemo(() => {
    return validateDate(formStore.toDate);
  });
  const fromToDateValidation = createMemo(() => {
    return validateFromToDate(formStore.fromDate, formStore.toDate);
  });
  const finalValidation = createMemo(() => {
    return (
      (formStore.flightType === "one-way-flight" &&
        fromDateValidation().isValid) ||
      (formStore.flightType === "return-flight" &&
        fromDateValidation().isValid &&
        toDateValidation().isValid &&
        fromToDateValidation().isValid)
    );
  });

  function handleSelectChange(e: Event) {
    const value = (e.target as HTMLSelectElement).value as FlightType;
    setFormStore("flightType", value);
    if (value === "one-way-flight") {
      setFormStore("toDate", "");
      setFormStore("errors", "toDate", undefined);
      toDateRef.value = "";
    }
  }

  function handleFromDateChange(e: Event) {
    const value = (e.target as HTMLInputElement).value;
    setFormStore("fromDate", value);
    setFormStore("errors", "fromDate", fromDateValidation().message);
  }

  function handleToDateChange(e: Event) {
    const value = (e.target as HTMLInputElement).value;
    setFormStore("toDate", value);
    setFormStore("errors", "toDate", toDateValidation().message);
    setFormStore("errors", "toDate", fromToDateValidation().message);
  }

  function handleSubmit(e: Event) {}

  return (
    <form class="flex flex-col gap-2 mb-8">
      <fieldset class="fieldset">
        <legend class="fieldset-legend">Type</legend>
        <select
          class="select"
          value={formStore.flightType}
          onChange={handleSelectChange}
        >
          <option value="one-way-flight">One-way flight</option>
          <option value="return-flight">Return flight</option>
        </select>
      </fieldset>
      <fieldset class="fieldset">
        <legend class="fieldset-legend">From Date (dd.mm.yyyy)</legend>
        <input
          class="input"
          value={formStore.fromDate}
          onInput={handleFromDateChange}
          ref={fromDateRef}
        />
        <Show when={formStore.errors.fromDate}>
          <span class="fieldset-label text-error">
            {formStore.errors.fromDate}
          </span>
        </Show>
      </fieldset>
      <fieldset class="fieldset">
        <legend class="fieldset-legend">To Date (dd.mm.yyyy)</legend>
        <input
          class="input"
          value={formStore.toDate}
          on:keyup={handleToDateChange}
          ref={toDateRef}
          disabled={formStore.flightType === "one-way-flight"}
        />
        <Show when={formStore.errors.toDate}>
          <span class="fieldset-label text-error">
            {formStore.errors.toDate}
          </span>
        </Show>
      </fieldset>
      <div class={"flex flex-row"}>
        <button
          class="btn"
          type="button"
          on:click={handleSubmit}
          disabled={!finalValidation()}
        >
          Submit
        </button>
      </div>
    </form>
  );
}
