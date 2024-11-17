<script lang="ts">
import "../app.css";
import { page } from "$app/stores";

let { children } = $props();

type SideItem = {
  routeId: string;
  label: string;
};

const sideItems: SideItem[] = [
  {
    routeId: "/",
    label: "Home",
  },
  {
    routeId: "/1-counter",
    label: "1. Counter",
  },
  {
    routeId: "/2-temperature-converter",
    label: "2. Temperature Converter",
  },
  {
    routeId: "/3-flight-booker",
    label: "3. Flight Booker",
  },
  {
    routeId: "/4-timer",
    label: "4. Timer",
  },
  {
    routeId: "/5-crud",
    label: "5. CRUD",
  },
  {
    routeId: "/6-circle-drawer",
    label: "6. Circle Drawer",
  },
  {
    routeId: "/7-cells",
    label: "7. Cells",
  },
];
</script>

<div class="flex min-h-screen">
  <!-- Sidebar -->
  <aside class="sidebar w-64 bg-base-200 text-base-content p-4">
    <h2 class="text-xl font-bold mb-4 w-full text-center">7GUIs</h2>
    <nav>
      <ul class="menu bg-base-200 w-56 space-y-2 rounded-box">
        {#each sideItems as item}
          <li>
            <a
              class="menu-item"
              class:active={$page.route.id === item.routeId}
              href={item.routeId}
            >
              {item.label}
            </a>
          </li>
        {/each}
      </ul>
    </nav>

  </aside>

  <!-- Main Content -->
  <main class="flex-1 p-6">
    <div class="container mx-auto">
      {@render children()}
    </div>
  </main>
</div>

