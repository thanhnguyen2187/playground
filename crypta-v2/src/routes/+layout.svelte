<script lang="ts">
import "../app.postcss";
import { AppShell, AppBar } from "@skeletonlabs/skeleton";

// Highlight JS
// import hljs from "highlight.js/lib/core";
// import "highlight.js/styles/github-dark.css";
// import { storeHighlightJs } from "@skeletonlabs/skeleton";
// import xml from "highlight.js/lib/languages/xml"; // for HTML
// import css from "highlight.js/lib/languages/css";
// import javascript from "highlight.js/lib/languages/javascript";
// import typescript from "highlight.js/lib/languages/typescript";

// hljs.registerLanguage("xml", xml); // for HTML
// hljs.registerLanguage("css", css);
// hljs.registerLanguage("javascript", javascript);
// hljs.registerLanguage("typescript", typescript);
// storeHighlightJs.set(hljs);

// Floating UI for Popups
import {
	computePosition,
	autoUpdate,
	flip,
	shift,
	offset,
	arrow,
} from "@floating-ui/dom";
import { storePopup } from "@skeletonlabs/skeleton";
storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });
</script>

<!-- App Shell -->
<AppShell>
	<svelte:fragment slot="header">
		<!-- App Bar -->
		<AppBar slotDefault="place-self-center">
			<svelte:fragment slot="lead">
				<strong class="text-xl uppercase">Crypta</strong>
			</svelte:fragment>
			<div class="input-group input-group-divider mb-2">
        <div class="input-group-shim">
          <i class="fa-solid fa-search"></i>
        </div>
        <input
          type="text"
          placeholder="Search..."
        />
      </div>
			<svelte:fragment slot="trail">
				<strong class="text-xl uppercase invisible">Crypta</strong>
			</svelte:fragment>
		</AppBar>
	</svelte:fragment>
	<!-- Page Route Content -->
	<slot />
</AppShell>
