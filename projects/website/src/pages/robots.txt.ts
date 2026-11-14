import type { APIRoute } from 'astro'

/**
 * AI crawlers are welcome: being read and cited by AI assistants and answer engines
 * is the goal (GEO). They are listed by name so the intent is explicit, for search,
 * user-triggered browsing and training alike.
 */
const aiCrawlers = [
  // OpenAI
  'GPTBot',
  'OAI-SearchBot',
  'ChatGPT-User',
  // Anthropic
  'ClaudeBot',
  'Claude-SearchBot',
  'Claude-User',
  // Perplexity
  'PerplexityBot',
  'Perplexity-User',
  // Google (Gemini) and Apple (Apple Intelligence)
  'Google-Extended',
  'Applebot-Extended',
  // Others
  'meta-externalagent',
  'Amazonbot',
  'DuckAssistBot',
  'MistralAI-User',
  'CCBot',
]

export const GET: APIRoute = ({ site }) => {
  const url = (path: string) => new URL(path, site).href
  const body = [
    'User-agent: *',
    'Allow: /',
    '',
    ...aiCrawlers.map((agent) => `User-agent: ${agent}`),
    'Allow: /',
    '',
    `# Site summary for LLMs: ${url('/llms.txt')}`,
    `# Every page as Markdown: ${url('/llms-full.txt')}`,
    `Sitemap: ${url('/sitemap-index.xml')}`,
    '',
  ].join('\n')

  return new Response(body, {
    headers: { 'Content-Type': 'text/plain; charset=utf-8' },
  })
}
