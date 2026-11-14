export const site = {
  name: 'Features',
  tagline: 'Explore the features of your codebase',
  description:
    'Features is an open-source, technology-agnostic CLI and web dashboard that maps the features of your codebase, their owners, history, test coverage and technical debt.',
  repository: 'https://github.com/interaction-dynamics/features',
  demo: 'https://interaction-dynamics.io/features/',
  npm: 'https://www.npmjs.com/package/features-cli',
  crate: 'https://crates.io/crates/features-cli',
  roadmap: 'https://github.com/orgs/interaction-dynamics/projects/18/views/4',
  organization: 'Interaction Dynamics',
  organizationUrl: 'https://interaction-dynamics.io',
  license: 'MIT',
}

export const nav = [
  { href: '/feature-based-architecture', label: 'Concept' },
  { href: '/guidelines', label: 'Guidelines' },
  { href: '/cli', label: 'CLI' },
  { href: '/faq', label: 'FAQ' },
]

export const githubUrl = (path: string) =>
  `${site.repository}/blob/master/${path}`
