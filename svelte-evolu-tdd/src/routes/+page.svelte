<script lang="ts">
import { machine } from "./state";
import { wrap } from "$lib/xstate-wrapper.svelte";
import { createActor } from "xstate";

let inputTask: HTMLInputElement;

const actor = wrap(createActor(machine));

const tasks = $derived(Array.from(actor.state.context.tasksMap.values()));
const tasksCompleted = $derived(tasks.filter((task) => task.done));
const tasksActive = $derived(tasks.filter((task) => !task.done));

function handleAddTask() {
  actor.ref.send({
    type: "Add",
    id: crypto.randomUUID(),
    name: inputTask.value,
  });
  inputTask.value = "";
}

function handleInputKeydown(event: KeyboardEvent) {
  if (event.key === "Enter") {
    handleAddTask();
  }
}

function handleRemoveTask(id: string) {
  actor.ref.send({ type: "Remove", id });
}

function handleCheckTask(id: string) {
  actor.ref.send({ type: "Check", id });
}

function handleUncheckTask(id: string) {
  actor.ref.send({ type: "Uncheck", id });
}
</script>

<div class="navbar bg-base-100 shadow-lg">
  <div class="flex-1">
    <button class="btn btn-ghost normal-case text-xl">My Tasks</button>
  </div>
</div>

<!-- Main Content -->
<div class="container mx-auto px-4 py-8 max-w-3xl">
  <!-- Add Task Form -->
  <div class="card bg-base-100 shadow-xl mb-8">
    <div class="card-body">
      <form class="flex gap-2">
        <input
          type="text"
          placeholder="Enter your task here"
          class="input input-bordered flex-1"
          bind:this={inputTask}
        />
        <button
          class="btn btn-primary"
          onclick={handleAddTask}
        >
          Add Task
        </button>
      </form>
    </div>
  </div>

  <!-- Task List -->
  <div class="card bg-base-100 shadow-xl">
    <div class="card-body">
      <!-- Tasks -->
      <ul class="space-y-4">
        <!-- Completed Task -->
        {#each tasksCompleted as task (task.id)}
          <li class="flex items-center justify-between">
            <div class="form-control">
              <label class="label cursor-pointer">
                <input
                  type="checkbox"
                  checked class="checkbox checkbox-primary"
                  onclick={() => handleUncheckTask(task.id)}
                />
                <span
                  class="label-text ml-4 line-through text-base-content/60"
                >
                  {task.name}
                </span>
              </label>
            </div>
            <button
              class="btn btn-ghost btn-circle btn-sm"
              aria-label="activate"
              onclick={() => handleRemoveTask(task.id)}
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </li>
        {/each}

        <!-- Active Tasks -->
        {#each tasksActive as task (task.id)}
          <li class="flex items-center justify-between">
            <div class="form-control">
              <label class="label cursor-pointer">
                <input
                  type="checkbox"
                  class="checkbox checkbox-primary"
                  onclick={() => handleCheckTask(task.id)}
                />
                <span class="label-text ml-4">{task.name}</span>
              </label>
            </div>
            <button
              class="btn btn-ghost btn-circle btn-sm"
              aria-label="activate"
              onclick={() => handleRemoveTask(task.id)}
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </li>
        {/each}
      </ul>

      <!-- Empty State (hidden by default) -->
      <div class="hidden text-center py-12">
        <div class="flex flex-col items-center gap-4">
          <div class="mask mask-squircle bg-base-200 p-4">
            <svg class="h-12 w-12 text-base-content/60" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
            </svg>
          </div>
          <div>
            <h3 class="font-bold text-lg">No tasks yet</h3>
            <p class="text-base-content/60">Get started by creating a new task.</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>
