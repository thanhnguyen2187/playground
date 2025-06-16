import { createSignal } from "solid-js";

export default function Counter() {
  const [count, setCount] = createSignal(0);

  return (
    <div class={"flex gap-2 items-center"}>
      <span>{count()}</span>
      <button
        class={"btn"}
        on:click={() => setCount(count() + 1)}
        type={"button"}
      >
        Increase
      </button>
    </div>
  );
}
