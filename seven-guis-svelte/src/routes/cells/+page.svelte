<script lang="ts">
  import { List } from "../../libs/lazy-list.js";

  const rowCount = 99
  const columnCount = 99

</script>

<div style="height: 600px; width: 1200px; overflow: scroll;">
  <table>
    {#each List.range(0, 100) as _}
      <tr>
        {#each List.range(0, 100) as cell}
          <td contenteditable="true"> </td>
        {/each}
      </tr>
    {/each}
  </table>
</div>

<style>
  table, th, td {
      border: 1px solid;
  }
  td {
      width: 100px;
      height: 2em;
  }
</style>
