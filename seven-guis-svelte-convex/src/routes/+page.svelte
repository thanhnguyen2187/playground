<script lang="ts">
const guiTasks = [
	{
		name: 'Counter',
		path: '/counter',
		description: 'Simple counter with increment functionality',
		implemented: true
	},
	{
		name: 'Temperature Converter',
		path: '/temperature',
		description: 'Convert between Celsius and Fahrenheit',
		implemented: false
	},
	{
		name: 'Flight Booker',
		path: '/flight-booker',
		description: 'Book one-way or return flights',
		implemented: false
	},
	{
		name: 'Timer',
		path: '/timer',
		description: 'Configurable timer with progress visualization',
		implemented: false
	},
	{
		name: 'CRUD',
		path: '/crud',
		description: 'Create, Read, Update, Delete operations',
		implemented: false
	},
	{
		name: 'Circle Drawer',
		path: '/circle-drawer',
		description: 'Draw and resize circles with undo/redo',
		implemented: false
	},
	{
		name: 'Cells',
		path: '/cells',
		description: 'Simple spreadsheet with formulas',
		implemented: false
	}
];
</script>

<div class="max-w-4xl">
	<div class="mb-8">
		<h1 class="text-4xl font-bold mb-4">7GUIs Implementation</h1>
		<p class="text-lg text-base-content opacity-80">
			A SvelteKit implementation of the 7GUIs benchmark using Convex for data management and DaisyUI for styling.
		</p>
	</div>

	<div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
		{#each guiTasks as task}
			<div class="card bg-base-100 shadow-lg border border-base-300">
				<div class="card-body">
					<div class="flex items-start justify-between">
						<h2 class="card-title text-lg">{task.name}</h2>
						{#if task.implemented}
							<div class="badge badge-success">✓</div>
						{:else}
							<div class="badge badge-outline">TODO</div>
						{/if}
					</div>
					<p class="text-sm text-base-content opacity-70 mb-4">{task.description}</p>
					<div class="card-actions justify-end">
						<a href={task.path} class="btn btn-primary btn-sm">
							{task.implemented ? 'View' : 'Build'}
						</a>
					</div>
				</div>
			</div>
		{/each}
	</div>

	<div class="mt-12 prose max-w-none">
		<h2>About 7GUIs</h2>
		<p>
			The 7GUIs benchmark is a collection of GUI programming challenges that test the expressiveness,
			ease of use, and clarity of a programming language and its GUI library. Each task represents
			a different aspect of GUI development.
		</p>

		<div class="alert alert-info mt-6">
			<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6">
				<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
			</svg>
			<span>This implementation uses SvelteKit 5, Convex for backend data management, and DaisyUI for component styling.</span>
		</div>
	</div>
</div>

