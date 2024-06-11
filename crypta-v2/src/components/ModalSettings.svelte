<script lang="ts">
import { ProgressRadial, Tab, TabGroup } from "@skeletonlabs/skeleton";
import Fa from "svelte-fa";
import { faConnectdevelop } from "@fortawesome/free-brands-svg-icons";
import { faCancel, faCheck, faClose, faEllipsis } from "@fortawesome/free-solid-svg-icons";

let tabActivated: "data" | "miscellanies" = "data";
let connectionState: "working" | "succeeded" | "failed" = "failed"
</script>

<div class="card p-4 pt-2 w-modal-slim">
  <TabGroup>
    <Tab bind:group={tabActivated} name="data" value={"data"}>
      <span>Data</span>
    </Tab>
    <Tab bind:group={tabActivated} name="miscellanies" value={"miscellanies"}>
      <span>Miscellanies</span>
    </Tab>
    <!-- Tab Panels --->
    <svelte:fragment slot="panel">
      <div class="flex flex-col gap-2 h-72">
        {#if tabActivated === "data"}
          <label class="label">
            <span>Connection URL</span>
            <input
              class="input"
              spellcheck="false"
            />
          </label>
          <label class="label">
            <span>Token</span>
            <input
              class="input"
              spellcheck="false"
            />
          </label>
          <div
            class="grow rounded flex flex-row justify-center items-center mt-4 p-2 gap-2"
            class:bg-success-800={connectionState === "succeeded"}
            class:bg-secondary-500={connectionState === "working"}
            class:bg-error-700={connectionState === "failed"}
          >
            {#if connectionState === "succeeded"}
              <span>Succeeded</span>
              <Fa icon={faCheck} />
            {:else if connectionState === "working"}
              <span>Connecting</span>
              <Fa icon={faEllipsis} />
            {:else if connectionState === "failed"}
              <span>Failed</span>
              <Fa icon={faClose} />
            {/if}
          </div>

          <div class="flex flex-row-reverse gap-2 mt-2">
            <button class="btn variant-filled-secondary">Save</button>
          </div>
        {:else if tabActivated === "miscellanies"}
          Hello world
        {/if}
      </div>
    </svelte:fragment>
  </TabGroup>
</div>


