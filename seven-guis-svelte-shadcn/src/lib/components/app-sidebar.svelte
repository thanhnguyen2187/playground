<script lang="ts">
import { parseAsString, useQueryState } from "nuqs-svelte";
import type { ComponentProps } from "svelte";
import * as Sidebar from "$lib/components/ui/sidebar/index.js";

let {
  ref = $bindable(null),
  ...restProps
}: ComponentProps<typeof Sidebar.Root> = $props();

const componentId = useQueryState(
  "component-id",
  parseAsString.withDefault("counter").withOptions({
    clearOnDefault: false,
  }),
);

const data = {
  versions: ["1.0.1", "1.1.0-alpha", "2.0.0-beta1"],
  navMain: [
    {
      title: "7GUIs Shadcn Svelte",
      url: "#",
      items: [
        {
          id: "counter",
          title: "01. Counter",
          url: "#",
        },
        {
          id: "temperature-converter",
          title: "02. Temperature Converter",
          url: "#",
        },
        {
          id: "flight-booker",
          title: "03. Flight Booker",
          url: "#",
        },
        {
          id: "todo",
          title: "04. TODO",
          url: "#",
        },
      ],
    },
  ],
};
</script>

<Sidebar.Root {...restProps} bind:ref>
	<Sidebar.Content>
		<!-- We create a Sidebar.Group for each parent. -->
		{#each data.navMain as group (group.title)}
			<Sidebar.Group>
				<Sidebar.GroupLabel>{group.title}</Sidebar.GroupLabel>
				<Sidebar.GroupContent>
					<Sidebar.Menu>
						{#each group.items as item (item.title)}
							<Sidebar.MenuItem>
								<Sidebar.MenuButton isActive={item.id === componentId.current}>
									{#snippet child({ props })}
										<a
											href={item.url}
											{...props}
											onclick={() => componentId.set(item.id)}
										>
											{item.title}
										</a>
									{/snippet}
								</Sidebar.MenuButton>
							</Sidebar.MenuItem>
						{/each}
					</Sidebar.Menu>
				</Sidebar.GroupContent>
			</Sidebar.Group>
		{/each}
	</Sidebar.Content>
	<Sidebar.Rail />
</Sidebar.Root>
