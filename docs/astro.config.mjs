// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

// https://astro.build/config
export default defineConfig({
	site: 'https://w3lt.github.io',
	base: '/devlog',
	integrations: [
		starlight({
			title: 'devlog',
			social: [
				{
					icon: 'github',
					label: 'GitHub',
					href: 'https://github.com/w3lt/devlog',
				},
			],
			sidebar: [
				{
					label: 'Getting started',
					items: [
						{ slug: 'getting-started/introduction' },
						{ slug: 'getting-started/installation' },
					],
				},
				{
					label: 'Guides',
					items: [
						{ slug: 'guides/usage' },
						{ slug: 'guides/projects' },
					],
				},
				{
					label: 'Reference',
					items: [
						{ slug: 'reference/data-storage' },
						{ slug: 'reference/project-layout' },
					],
				},
				{ label: 'Contributing', slug: 'contributing' },
			],
		}),
	],
});
