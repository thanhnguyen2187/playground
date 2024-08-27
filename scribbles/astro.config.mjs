import {defineConfig} from 'astro/config';
import starlight from '@astrojs/starlight';

// https://astro.build/config
export default defineConfig({
    integrations: [
        starlight({
            title: 'Thanh\'s Scribbles',
            social: {
                github: 'https://github.com/thanhnguyen2187/scribbles',
            },
            sidebar: [
                {
                    label: 'Software Design for Flexibility',
                    autogenerate: {directory: 'sdf'},
                },
                {
                    label: 'Guides',
                    items: [
                        // Each item here is one entry in the navigation menu.
                        {label: 'Example Guide', slug: 'guides/example'},
                    ],
                },
                {
                    label: 'Reference',
                    autogenerate: {directory: 'reference'},
                },
            ],
        }),
    ],
});
