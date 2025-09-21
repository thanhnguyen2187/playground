<script lang="ts">
import "../app.css";
import { setupConvex } from "convex-svelte";
import { page } from "$app/stores";
import { PUBLIC_CONVEX_URL } from "$env/static/public";
import favicon from "$lib/assets/favicon.svg";

setupConvex(PUBLIC_CONVEX_URL);

let { children } = $props();

const guiTasks = [
  { name: "Counter", path: "/counter" },
  { name: "Temperature Converter", path: "/temperature" },
  { name: "Flight Booker", path: "/flight-booker" },
  { name: "Timer", path: "/timer" },
  { name: "CRUD", path: "/crud" },
  { name: "Circle Drawer", path: "/circle-drawer" },
  { name: "Cells", path: "/cells" },
];
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<div class="drawer lg:drawer-open">
	<input id="drawer-toggle" type="checkbox" class="drawer-toggle" />
	<div class="drawer-content flex flex-col">
		<!-- Navbar for mobile -->
		<div class="navbar bg-base-300 lg:hidden">
			<div class="flex-none">
				<label for="drawer-toggle" aria-label="open sidebar" class="btn btn-square btn-ghost">
					<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block h-6 w-6 stroke-current">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>
					</svg>
				</label>
			</div>
			<div class="flex-1">
				<a href="/" class="btn btn-ghost text-xl">7GUIs</a>
			</div>
		</div>

		<!-- Main content -->
		<main class="flex-1 p-6">
			{@render children?.()}
		</main>
	</div>

	<!-- Sidebar -->
	<div class="drawer-side">
		<label for="drawer-toggle" aria-label="close sidebar" class="drawer-overlay"></label>
		<aside class="bg-base-200 min-h-full w-80 p-4">
			<div class="mb-8">
				<h1 class="text-2xl font-bold text-primary">7GUIs</h1>
				<p class="text-sm text-base-content opacity-70">Svelte + Convex Implementation</p>
			</div>

			<ul class="menu space-y-2">
				<li>
					<a href="/" class="{'/' === $page.url.pathname ? 'active' : ''}">
						<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
						</svg>
						Home
					</a>
				</li>
				{#each guiTasks as task}
					<li>
						<a href={task.path} class="{task.path === $page.url.pathname ? 'active' : ''}">
							{task.name}
						</a>
					</li>
				{/each}
			</ul>
		</aside>
	</div>
</div>
