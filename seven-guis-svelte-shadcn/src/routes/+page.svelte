<script lang="ts">
import { parseAsString, useQueryState } from "nuqs-svelte";
import AppSidebar from "$lib/components/app-sidebar.svelte";
import * as Breadcrumb from "$lib/components/ui/breadcrumb";
import { Button } from "$lib/components/ui/button/index";
import { Input } from "$lib/components/ui/input/index";
import { Separator } from "$lib/components/ui/separator";
import * as Sidebar from "$lib/components/ui/sidebar";

const componentId = useQueryState(
  "component-id",
  parseAsString.withDefault("counter").withOptions({
    clearOnDefault: false,
  }),
);

function mapComponentId(value: string) {
  switch (value) {
    case "counter":
      return "01. Counter";
    case "temperature-converter":
      return "02. Temperature Converter";
    case "flight-booker":
      return "03. Flight Booker";
  }
  return "00. TODO";
}

let pageTitle = $derived(mapComponentId(componentId.current));

let count = $state(0);
</script>

<Sidebar.Provider>
	<AppSidebar />
	<Sidebar.Inset>
		<header class="flex h-16 shrink-0 items-center gap-2 border-b px-4">
			<Sidebar.Trigger class="-ml-1" />
			<Separator orientation="vertical" class="mr-2 h-4" />
			<Breadcrumb.Root>
				<Breadcrumb.List>
					<Breadcrumb.Item class="hidden md:block">
						<Breadcrumb.Link href="#">7 GUIs Shadcn Svelte</Breadcrumb.Link>
					</Breadcrumb.Item>
					<Breadcrumb.Separator class="hidden md:block" />
					<Breadcrumb.Item>
						<Breadcrumb.Page>{pageTitle}</Breadcrumb.Page>
					</Breadcrumb.Item>
				</Breadcrumb.List>
			</Breadcrumb.Root>
		</header>
		<div class="flex flex-1 flex-col gap-4 p-4">
			<div class="flex gap-4 w-40">
				<Button
					variant="secondary"
					onclick={() => count = Number(count) + 1}
				>
					Increase
				</Button>
				<Input bind:value={count} />
			</div>
		</div>
	</Sidebar.Inset>
</Sidebar.Provider>
