<script lang="ts">
import "../app.postcss";
import {
	AppShell,
	AppBar,
	storePopup,
	initializeStores,
	Toast,
	Modal,
} from "@skeletonlabs/skeleton";
import { Fa } from "svelte-fa";
import { faSearch } from "@fortawesome/free-solid-svg-icons";
import {
	computePosition,
	autoUpdate,
	flip,
	shift,
	offset,
	arrow,
} from "@floating-ui/dom";

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
initializeStores();
storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });
</script>

<Toast/>
<Modal/>
<!-- App Shell -->
<AppShell>
	<svelte:fragment slot="header">
		<!-- App Bar -->
		<AppBar slotDefault="place-self-center">
			<svelte:fragment slot="lead">
				<strong class="text-xl uppercase">Crypta</strong>
			</svelte:fragment>
			<div class="input-group input-group-divider grid-cols-[auto_1fr]">
        <div class="input-group-shim">
					<Fa icon={faSearch} />
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
