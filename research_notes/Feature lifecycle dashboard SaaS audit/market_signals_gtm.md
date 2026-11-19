# Market signals and go-to-market evidence: per-feature lifecycle dashboard SaaS (on top of features-cli)

Research date: 2026-11-19. Each finding is dated where the date matters. Material from before 2025 is flagged. Several sources are vendor blogs or market-research resellers, and are marked as such. Some sites could not be reached from this research session: npmjs.com and api.npmjs.org returned 403, hn.algolia.com returned 403, and sleuth.io would not resolve. The gaps those caused are listed under each question.

## 1. Market size and growth (SEI / DPI, VSM, developer portals, product management software)

### Takeaway
The adjacent categories are real and maturing, but they are small and narrowly measured. Gartner moved "software engineering intelligence" from a Market Guide (2024) to a renamed "Developer Productivity Insight" Market Guide (2025) and then a first Magic Quadrant (2026), which signals category legitimacy. Independent dollar sizings put VSM at only about $0.5B, growing roughly 8-10% a year. No credible 2025-2026 dollar figure was found for SEI/DPI or for developer portals specifically.

### Cited Findings
- Gartner published its Market Guide for Software Engineering Intelligence Platforms on 13 March 2024 (pre-2025). It described the market as small but growing, with many vendors offering similar capabilities — [Gartner abstract 5276563](https://www.gartner.com/en/documents/5276563); summary in [Typo blog](https://typoapp.io/blog/gartners-report-on-software-engineering-intelligence-platforms-2024)
- Gartner published a "Market Guide for Developer Productivity Insight Platforms" on 12 May 2025, which in effect renamed and widened SEI. Vendor pages cite it as presenting these platforms as a "single source of truth" for engineering process data — [Gartner abstract 6455107](https://www.gartner.com/en/documents/6455107); [LinearB reprint page](https://linearb.io/resources/gartner-dpi-market-guide)
- 2026 saw Gartner's first Magic Quadrant for Developer Productivity Insight Platforms, with LinearB announcing it was named a Leader — [LinearB](https://linearb.io/resources/gartner-magic-quadrant-dpi-platforms-2026); [Swarmia blog](https://www.swarmia.com/blog/2026-gartner-magic-quadrant-developer-productivity-insight/)
- Gartner adoption predictions, all quoted secondhand by vendors or forums and not checked against Gartner text: "50% of software engineering organizations will use SEI platforms by 2027, up from 5% in 2024" — [tianpan.co forum](https://tianpan.co/forum/t/gartner-50-of-engineering-orgs-will-use-intelligence-platforms-by-2027-the-10x-adoption-wave-is-here/311). "By 2028, 60% of Fortune 500 companies will use DPI platforms, up from 15%" — [LinearB](https://linearb.io/resources/gartner-dpi-market-guide). The two baselines do not match.
- Developer portals: Gartner's 2025 Market Guide for Internal Developer Portals reportedly forecasts that 85% of organizations with platform engineering teams will provide internal developer portals by 2028, up from 60% in 2025. This is quoted secondhand — [Roadie blog](https://roadie.io/blog/platform-engineering-in-2026-why-diy-is-dead/); [Cortex blog](https://www.cortex.io/post/cortex-recognized-again-as-a-representative-vendor-in-the-2025-gartner-market-guide-for-internal-developer-portals)
- Value Stream Management: Grand View Research estimates $480.5M in 2024, growing at a 9.8% CAGR over 2025-2030 — [Grand View via GII](https://www.giiresearch.com/report/grvi1751311-value-stream-management-market-size-share-trends.html). Fortune Business Insights estimates $535.96M in 2025, $589.32M in 2026 and $1,259.4M by 2034, a 9.96% CAGR — [Fortune Business Insights](https://www.fortunebusinessinsights.com/value-stream-management-market-117477). Market Research Forecast estimates $457.9M for VSM software in 2025 at an 8.3% CAGR — [MRF](https://www.marketresearchforecast.com/reports/value-stream-management-software-39022). Other resellers quote CAGRs of 12.5-25%, a spread that suggests low reliability — [Custom Market Insights via Winchester Sun](https://smb.winchestersun.com/article/Latest-Global-Value-Stream-Management-Market-SizeShare-Worth-USD-89-Billion-by-2034-at-a-125percent-CAGR-Custom-Market-Insights-Analysis-Outlook-Leaders-Report-Trends-Forecast-Segmentation-Growth-Growth-Rate-Value/6a57eea316e7610b3c53f32b)
- Product management software: the only market report found had its size and CAGR fields blank — [marketpublishers.com](https://marketpublishers.com/report/it-technology/other-ict-n-software/global-project-tracking-tools-market-2022-by-company-regions-type-n-application-forecast-to-2028.html)
- Gartner survey (May 2025): 77% of engineering leaders say integrating AI into applications is a major challenge. This is indirect context on where engineering leaders' attention is going — [Gartner press release](https://www.gartner.com/en/newsroom/press-releases/2025-05-22-gartner-survey-finds-77-percent-of-engineering-leaders-identify-ai-integration-in-apps-as-a-major-challenge)

### Inferences
- A "per-feature lifecycle dashboard" has no analyst category of its own. It sits between DPI/SEI (engineering-side metrics), internal developer portals (service catalog and ownership) and product analytics or product management (PostHog, Productboard). Buyers will compare it against those budgets rather than fund a new line item.
- Category signals are rising: a renamed category, a first Magic Quadrant and a 5% to 50% adoption forecast. Measured dollar markets are small, though. VSM is around $0.5B, and SEI vendor revenues are mostly private. The opportunity is more likely a wedge into one of these categories than a large standalone market.

### Gaps
- No Gartner, Forrester or IDC dollar sizing for SEI/DPI or for internal developer portals was found in public sources for 2025-2026. The Gartner documents are paywalled.
- No Forrester or IDC public figures for VSM or developer portals in 2025-2026 were found.
- No reliable 2025-2026 sizing for product management software was found.

## 2. Funding rounds, acquisitions and shutdowns, 2023-2026

### Takeaway
Strategic buyers and growth investors are paying heavily for platforms that already sit in daily engineering workflows or own a key data layer: Atlassian–DX (about $1B), OpenAI–Statsig ($1.1B), Port ($800M valuation), Linear ($1.25B, then a reported $2.5B). Visualization-only "understand your codebase" tools were folded into larger products (CodeSee into GitKraken), and a general project tool (Height) shut down.

### Cited Findings
- **Atlassian–DX (Sept 2025):** Atlassian agreed to buy DX, a developer intelligence and productivity platform, for about $1B in cash and restricted stock, its largest acquisition to date. Most DX enterprise customers (for example Pfizer, Pinterest, Xero) were already Atlassian customers, and DX is being folded into Jira, Rovo Dev and Bitbucket. Closing was expected in Q2 of Atlassian's fiscal 2026 — [Constellation Research](https://www.constellationr.com/insights/news/atlassian-buys-dx-1-billion); [BNN Bloomberg/Reuters](https://www.bnnbloomberg.ca/business/2025/09/18/atlassian-to-acquire-developer-productivity-platform-dx-for-us1-billion/). One outlet's "$1.5 billion" is the AUD figure — [InnovationAus](https://www.innovationaus.com/?p=68690)
- **Port (Dec 2025):** $100M Series C at an $800M valuation, led by General Atlantic with Accel, Bessemer and Team8. Total funding is $158M. This followed a $35M Series B earlier in 2025. Port positions itself as replacing do-it-yourself Backstage and is pivoting to an "agentic engineering platform" — [TechCrunch](https://techcrunch.com/2025/12/11/port-raises-100m-at-800m-valuation-to-take-on-spotifys-backstage); [SiliconANGLE](https://siliconangle.com/2025/12/11/port-nets-100m-turn-developer-portal-agentic-ai-hub/); [Port blog](https://www.port.io/blog/port-100m-series-c)
- **Cortex (Sept 2024, pre-2025):** $60M Series C led by Scale Venture Partners. No 2025-2026 round was found — [Scale VP](https://www.scalevp.com/press/cortex-raises--60m-to-build-the-future-of-developer-experience--series-c)
- **LinearB:** total funding is about $71M, with the last priced round a $50M Series B in May 2022 (pre-2025). No 2025-2026 round was found, and its revenue strategy has shifted toward metered AI code-review automation — [Sacra](https://sacra.com/c/linearb)
- **Jellyfish:** total funding is reported at about $114.5M by an aggregator, with round details unverified — [Sacra](https://sacra.com/c/jellyfish)
- **OpenAI–Statsig (Sept 2025):** an all-stock deal of about $1.1B. Statsig's founder became OpenAI's CTO of Applications. Statsig had raised a $100M Series C in May 2025 — [Bloomberg](https://www.bloomberg.com/news/articles/2025-09-02/openai-to-buy-product-testing-startup-statsig-for-1-1-billion); [YourStory](https://yourstory.com/ai-story/openai-statsig-acquisition-leadership-reshuffle)
- **Linear:** $82M Series C at a $1.25B valuation in June 2025, led by Accel — [TechCrunch](https://techcrunch.com/2025/06/10/atlassian-rival-linear-raises-82m-at-1-25b-valuation/); [Linear blog](https://linear.app/now/building-our-way). Dealroom reported a $99M employee tender at a $2.5B valuation in Aug 2026. That was secondary money, not new capital — [Dealroom](https://dealroom.co/news/147084-linear-doubles-valuation-to-2-5b-defying-the-saas-is-dead-narrative/)
- **CodeSee:** a codebase-visualization and "understand your code" tool. GitKraken acquired it in May 2024 (pre-2025) and folded it into GitKraken DevEx, and CodeSee ended its standalone commercial operations — [GitKraken blog](https://www.gitkraken.com/blog/gitkraken-launches-devex-platform-acquires-codesee); [AlternativeTo](https://alternativeto.net/news/2024/5/gitkraken-acquires-codesee-and-launches-new-developer-experience-platform-named-devex)
- **Height:** a project management tool. It announced its shutdown in March 2025 and ceased operations on 24 Sept 2025, after about 3.5 years of public availability — [AlternativeTo](https://alternativeto.net/news/2025/3/height-project-management-tool-to-shut-down-by-september-2025/)

### Inferences
- **Lesson 1 (distribution beats features):** DX sold to Atlassian because nearly all its customers already ran Jira. Strategic value went to whoever owned the system of record. A feature dashboard that depends on Jira, GitHub and Sentry data is exposed to those platforms adding "feature views" themselves.
- **Lesson 2 (visualization alone is thin):** CodeSee, a code-map and visualization tool and the closest analog to "map the codebase", did not become a standalone business and was absorbed. features-cli's codebase map risks the same fate unless it becomes a workflow, through the agent and MCP layer, CI checks and ownership enforcement.
- **Lesson 3 (AI-agent narrative is the funding theme):** Port, Statsig and Linear all raised or exited on AI and agent narratives in 2025-2026. A "feature context for AI agents" angle, such as features-cli's MCP server, fits current investor framing better than "dashboard".
- **Lesson 4:** Height shows that crowded horizontal tools without a wedge die even with good design.

### Gaps
- No public 2025-2026 funding rounds were found for Cortex, LinearB or Jellyfish. Their revenue figures are private.
- The DX deal's close was not confirmed in these results. It was expected by end of calendar 2025.

## 3. Budget owner for "feature visibility" and accepted price points

### Takeaway
Engineering-intelligence and portal tools are bought by engineering leadership (VP Engineering, CTO, platform engineering heads) and priced per contributing developer. Public prices range from about $20 to $59 per developer per month, with free tiers below 10-15 developers. Jellyfish-style enterprise deals are quote-based and suit companies with more than 200 developers. No source found showed a CPO-owned budget for this specific need.

### Cited Findings
- LinearB lists $29 and $59 per contributor per month, with unlimited free viewer seats. One source cites a 50-contributor minimum on Essentials — [Sacra](https://sacra.com/c/linearb); [rywalker.com](https://rywalker.com/research/linearb)
- Swarmia lists about $45 (Standard) and $55 (Enterprise) per developer per month, billed annually, and is free under 10 developers. Modules sell separately at $5-$23 per developer per month (AI adoption $5, surveys $9, software capitalization $18, productivity and AI impact $23). Older listings show lower Lite prices — [Toolradar](https://toolradar.com/tools/swarmia/pricing); [Sacra](https://sacra.com/c/swarmia)
- Port charges per "contributing developer", meaning anyone who committed in the last 90 days. It has a free plan up to 15 seats, Basic at about $30 per seat per month (up to 50 seats), Standard at about $40 (up to 200 seats), and custom Enterprise pricing — [Port pricing](https://port.io/pricing); [Toolradar](https://toolradar.com/tools/port/pricing)
- Jellyfish does not publish prices. It sells quote-based, by contributor count and module, and one comparison says it "does not make sense below 200 developers" — [rywalker.com](https://rywalker.com/research/jellyfish); [Toolradar comparison](https://toolradar.com/compare/jellyfish-vs-linearb)
- DX's customer base and buyers were engineering organizations already on Atlassian, and Atlassian bought it to measure engineering productivity and AI adoption — [Constellation Research](https://www.constellationr.com/insights/news/atlassian-buys-dx-1-billion)
- Swarmia's software-capitalization module suggests some of this spend also serves finance and CFO reporting needs — [Toolradar](https://toolradar.com/tools/swarmia/pricing)

### Inferences
- The likely economic buyer is the VP Engineering or CTO, or a platform engineering lead for ownership and catalog use cases. CPOs buy product analytics and roadmapping tools such as PostHog, Amplitude and Productboard, which are a separate budget. A "design to production" dashboard that spans both risks having no single owner.
- An anchor price of about $20-40 per active developer per month is defensible against Port, LinearB and Swarmia. A 50-developer team at $25 per developer per month comes to about $15k per year. Reaching $1M ARR would take roughly 70 such teams.
- Per-module pricing at Swarmia shows buyers will pay $5-20 per developer for a single narrow insight, such as AI impact. A "feature ownership and health" module may be easier to sell than a whole platform.

### Gaps
- No survey data was found on who owns the budget for "feature visibility" specifically, comparing VP Eng, CPO and CTO.
- Several prices come from aggregator sites (Toolradar, Sacra, rywalker.com) and may be out of date. Vendor pages were not fetched directly, except Port's, which appeared in the search results.

## 4. Pain evidence: losing track of features, bloat, ownership, AI-driven code growth

### Takeaway
There is strong 2025 evidence that AI raises code output and churn while also raising instability, duplication and distrust. That supports a "you are losing track of what you ship" narrative. No survey was found that measures "losing track of features" or feature-level ownership confusion directly, so the pain is inferred and not measured.

### Cited Findings
- **DORA 2025 (State of AI-assisted Software Development, Sept 2025):** nearly 5,000 respondents, 90% using AI at work. AI adoption now correlates with higher delivery throughput, a reversal from 2024, but software delivery instability keeps rising with AI adoption. The report frames AI as an "amplifier" of existing organizational strengths and weaknesses — [Google Cloud DORA PDF](https://services.google.com/fh/files/misc/2025_state_of_ai_assisted_software_development.pdf); [InfoQ](https://www.infoq.com/news/2025/09/dora-state-of-ai-in-dev-2025); [DevOps.com](https://devops.com/dora-2025-faster-but-are-we-any-better/)
- **GitClear 2025 AI Copilot Code Quality report:** based on 211M changed lines from 2020 to 2024. "Moved" (refactored) code fell from 24.8% of changed lines in 2021 to 9.5% in 2024. Copy/pasted code rose from 8.4% to 12.3%. Duplicated blocks of five or more lines rose eightfold in 2024. 2024 was the first year copy/paste exceeded moved code. Caveat: the report shows correlation with the spread of AI assistants, not proof of attribution — [LeadDev](https://leaddev.com/software-quality/how-ai-generated-code-accelerates-technical-debt); [DevClass](https://www.devclass.com/ai-ml/2025/02/20/ai-is-eroding-code-quality-states-new-in-depth-report/1626250). The 2024 DORA report had found AI linked to a 3.4% code-quality improvement, which conflicts with GitClear — [The New Stack](https://thenewstack.io/whats-missing-with-ai-generated-code-refactoring)
- **Stack Overflow Developer Survey 2025:** 49,009 responses, fielded May-June 2025. 84% use or plan to use AI tools, up from 76%. 46% distrust AI accuracy against 33% who trust it, and only 3% highly trust it. 66% name "AI solutions that are almost right, but not quite" as their top frustration. The second most common complaint is that debugging AI code is more time-consuming. Sources differ on the exact trust percentages — [TechRepublic](https://www.techrepublic.com/article/news-developer-trust-in-ai-declines/); [ShiftMag](https://shiftmag.dev/stack-overflow-survey-2025-ai-5653/)
- **Ownership and portals:** Spotify's Backstage lead was quoted saying many external adopters get stuck at about 10% internal adoption, against 99% inside Spotify. This is anecdotal and widely repeated, with no Gartner source found — [byteiota](https://byteiota.com/backstages-89-idp-market-share-hides-10-adoption-crisis/); [tianpan.co](https://tianpan.co/forum/t/backstage-holds-89-market-share-but-only-10-internal-adoption-why/417)

### Inferences
- The best-evidenced pain is AI-accelerated change volume combined with rising instability and duplication. Engineering leaders want to know what was shipped, who owns it, whether it is tested, and whether it is used. That maps directly onto features-cli's owners, coverage, debt and PostHog/Sentry usage signals.
- The "unused features / feature bloat" angle had no fresh 2025-2026 data in this research. The widely cited Pendo or Standish "80% of features rarely used" claims were not verified here and should not be quoted without sourcing.

### Gaps
- No 2025-2026 survey quantifying "losing track of features", unused features, or ownership confusion was found.
- The exact GitClear 2026 follow-up figures (for example, the projection of refactoring at about 3-4%) are projections in commentary, not verified measurements.
- Hacker News and Reddit discussion searches could not be run, because hn.algolia.com was blocked from this session.

## 5. Why "single pane of glass" and end-to-end dashboards fail

### Takeaway
The recurring failure modes are integration and data-quality burden, low trust in the numbers, unclear ownership of connectors, and the dashboard not being anyone's daily tool. The Backstage "10% adoption" anecdote and Gartner's warning that Backstage is "misconstrued as ready-to-use" are the most relevant engineering analogs. Rigorous studies are scarce; most evidence is practitioner or vendor commentary.

### Cited Findings
- Gartner's 2025 IDP Market Guide reportedly says organizations "misconstrue Backstage as a ready-to-use portal that requires no additional work". Teams burn out on maintaining it before delivering features developers want. This is quoted secondhand via vendor blogs — [Roadie](https://roadie.io/blog/platform-engineering-in-2026-why-diy-is-dead/); [Cortex](https://www.cortex.io/post/an-overview-of-spotify-backstage)
- Backstage reaches about 10% internal adoption at typical organizations, an anecdote attributed to Spotify's Backstage lead — [byteiota](https://byteiota.com/backstages-89-idp-market-share-hides-10-adoption-crisis/)
- SRE leadership essay "The Single Pain of Glass": cramming everything into one view "has shown itself to be a recipe for being completely ignored". It recommends views that show where and why a problem exists — [Medium, SRE Leadership](https://medium.com/site-reliability-engineering-leadership/the-single-pain-of-glass-6e42930e966)
- Dashboard abandonment: up to 90% of BI dashboards go unused after six months, and only 16% of organizations reach full adoption. These figures are weakly sourced, from vendor blog aggregation — [OWOX](https://www.owox.com/blog/articles/why-dashboards-fail)
- Trust as the root cause: leaders think they have a data quality issue when the real problem is trust, and dashboards get abandoned when people doubt the outputs — [VentureBeat](https://venturebeat.com/infrastructure/data-everywhere-alignment-nowhere-what-dashboards-are-getting-wrong-and-why-you-need-a-data-product-manager)
- Engineers routing work around a system rather than through it degraded data quality in a Ford rollout case study — [IndustryWeek](https://www.industryweek.com/technology-and-iiot/automation/article/55375248/automation-wont-save-you-if-nobody-uses-it-correctly)
- Sleuth (an engineering-metrics vendor) published an "engineering dashboards post-mortem", but it could not be fetched (host blocked) — [Sleuth](https://sleuth.io/post/eng-dashboards-post-mortem)

### Inferences
- An "end-to-end per-feature dashboard" combines the hardest parts: mapping code to features (taxonomy drift), linking specs, tickets and analytics events to the same feature IDs, and keeping everything current. features-cli's approach of feature metadata living in the repo (README frontmatter, `__docs__/specs`, CODEOWNERS generation) partly avoids connector-maintenance burden, because the code is the source of truth. That is a real differentiator against connector-heavy portals.
- To avoid being "nobody's daily tool", the product should push signals into existing surfaces, such as PR checks, CODEOWNERS, AI agent context via MCP, and Slack or Linear, rather than rely on people visiting a dashboard.

### Gaps
- No formal post-mortem specific to an end-to-end feature lifecycle dashboard product was found.
- The Sleuth post-mortem content could not be retrieved.

## 6. OSS-to-SaaS playbooks (PostHog, Sentry, GitLab, Backstage to Roadie/Spotify Portal, Infracost, Codecov)

### Takeaway
The successful playbooks treat open source as distribution and monetize hosting plus team and enterprise features: history, collaboration, SSO/RBAC/audit, compliance and support. PostHog's own conclusion is that open source is a great distribution channel but "doesn't make for great monetization". Backstage shows a commercial opening for managed or opinionated versions of a do-it-yourself OSS tool (Roadie, Port, Cortex).

### Cited Findings
- PostHog started with a paid hosted edition, moved to open core with self-hosting, and later stopped selling self-hosted licenses. Its team concluded open source is "a great initial distribution channel but doesn't make for great monetization" — [PostHog blog](https://posthog.com/blog/open-source-business-models)
- PostHog's paid add-ons gate SSO, RBAC, SLAs and support. A third-party teardown lists Boost at $250/mo, Scale at $750/mo and Enterprise at $2,000/mo (unverified) — [DEV, PostHog pricing teardown 2026](https://dev.to/beton/posthog-pricing-teardown-2026-57oo). The described funnel runs from free self-hosted, to outgrowing infrastructure, to PostHog Cloud — [Sacra](https://sacra.com/chat/h/4520c485-376b-47ca-bc6d-d11e04e18170/)
- Sentry started hosting in 2012 because self-hosting was "a huge pain to maintain". The SaaS funds compliance and security features that large organizations need — [Sentry](https://sentry.io/resources/the-future-of-open-source-saas-the-final-frontier); [Sentry blog](https://blog.sentry.io/sentry-thrives-open-source-software-company)
- Sentry acquired Codecov in 2022 (pre-2025), an example of a developer metric tool (coverage) becoming a feature of a larger platform — [Sentry blog search result](https://blog.sentry.io/sentry-thrives-open-source-software-company)
- Common open-core split: the core under MIT, with SSO, RBAC and audit logs in a separately licensed enterprise directory (PostHog, Infisical, Chatwoot). The jump from a roughly $20/mo plan to a $25k/yr commitment is usually triggered by SSO or compliance, not features — [youngju.dev](https://www.youngju.dev/blog/culture/2026-04-12-open-source-monetization.en); [Open Core Ventures](https://www.opencoreventures.com/blog/building-venture-scale-open-core)
- Backstage commercial layer: Port explicitly positions against "build-it-yourself Backstage" ($800M valuation), and Roadie argues "DIY is dead" for portals in 2026 — [OpenSourceForU](https://www.opensourceforu.com/2025/12/port-secures-100m-to-replace-build-it-yourself-backstage-with-a-managed-ai-agent-stack/); [Roadie](https://roadie.io/blog/platform-engineering-in-2026-why-diy-is-dead/)

### Inferences
- Applied to features-cli, the natural paid line is:
  - Free: the CLI, local web UI and MCP server.
  - Paid: hosted multi-repo aggregation and history (trend of feature health over time, which a stateless CLI cannot provide), team-level ownership views, PR and CI checks across repos, persistent PostHog/Sentry sync, and SSO, RBAC and audit for enterprise.
- PostHog and Sentry converted because their OSS products were heavy to self-host, with databases and ingestion. A stateless local CLI has a much weaker "outgrow self-hosting" trigger, so the hosted value must come from cross-repo, historical and collaborative features, not from relieving infrastructure pain.
- The MIT license allows anyone, including Port, Cortex or GitKraken, to embed the CLI. That helps distribution but offers no moat.

### Gaps
- No usable first-party data on Infracost's OSS-to-paid conversion was found. Searches returned nothing relevant.
- No public conversion-rate benchmarks (OSS user to paid) for these companies were found.
- GitLab's and Spotify Portal for Backstage's commercial details were not researched within the tool budget.

## 7. Demand signals for features-cli itself (public pages only)

### Takeaway
Public demand signals are very early-stage. As of 19 Nov 2026, the project has 11 GitHub stars, 0 forks, 0 watchers, about 4.2k total crates.io downloads (140 recent), and no visible external issues or contributors. Development activity is very high, with over 200 PRs in about 13 months, but almost entirely from a single maintainer plus Dependabot. No HN or Reddit mentions were found.

### Cited Findings
- GitHub `interaction-dynamics/features`: 11 stars, 0 forks, 0 subscribers, 6 open issues and PRs (including Dependabot PRs). Created 2025-10-08 and last pushed 2026-11-18. MIT license. Description: "The CLI to explore the features in your code and improve their ownership" — [GitHub API](https://api.github.com/repos/interaction-dynamics/features)
- Of the latest 100 issues and PRs, all are pull requests (numbered up to #206), from 2 distinct authors (the maintainer and Dependabot). Recent PRs (Nov 2026) add an MCP server (`features mcp`), a feature health score and risk matrix, an owner page, skills install, and a public changelog — [GitHub issues API](https://api.github.com/repos/interaction-dynamics/features/issues?state=all&per_page=100)
- crates.io `features-cli`: 4,209 total downloads, 140 recent (crates.io's "recent" is a 90-day window). Created 2025-10-10, max version 0.14.0, last updated 2026-11-06 — [crates.io API](https://crates.io/api/v1/crates/features-cli). Many docs.rs versions exist (0.1.0 to 0.14) — [docs.rs](https://docs.rs/crate/features-cli/0.11.2/source/Cargo.toml)
- The project is indexed in ecosyste.ms "awesome" listings — [ecosyste.ms](https://awesome.ecosyste.ms/projects/github.com%2Finteraction-dynamics%2Ffeatures)
- npm `features-cli` weekly downloads: not retrievable, because npmjs.com and api.npmjs.org returned 403 from this session.
- A web search for "features-cli" found only docs.rs and ecosyste.ms pages, with no HN, Reddit, blog or social mentions — [web search results, docs.rs](https://docs.rs/crate/features-cli/0.9.6/source/README.md)

### Inferences
- Total crates.io downloads are inflated by frequent releases and automated mirrors and CI. With 140 downloads in 90 days and 11 stars, the organic user base is likely in the tens, not hundreds.
- There is no external pull yet: no third-party issues, no contributors, no forks. Before building a SaaS, the founder lacks validation that teams adopt the CLI. The standard OSS-to-SaaS precondition, a meaningful base of free users who ask for team or hosted features, is not yet met.
- A cheap next step would be a launch (Show HN, r/programming or r/ExperiencedDevs, the AI agent and MCP communities), plus measuring stars, downloads, inbound issues and design-partner interest before investing in hosting.

### Gaps
- npm weekly downloads (blocked, 403).
- Hacker News mentions via the Algolia API (blocked, 403). Reddit was not checked.
- The GitHub traffic API (views and clones) requires owner authentication and was not accessed.
