# Engineering intelligence, developer portals and codebase-understanding tools as competitors to a per-feature lifecycle dashboard

Research date: 2026-11-19. Method note: vendor sites (swarmia.com, linearb.io, codescene.com, port.io) could not be fetched from this research session (network allowlist), so all findings come from web-search result summaries. Many cited pages are competitor-written comparison blogs or aggregators (PitchBook/Tracxn/CB Insights snippets). Treat the pricing and funding figures as indicative, and check primary sources before using them in a pitch deck. Anything marked "unverified" is an unconfirmed lead, not a fact.

## 1. Which tools have a first-class "feature" entity, and how do they define it?

### Takeaway
I found no tool in these three categories that makes a code-level "feature" (a folder or module mapped to owners, debt, coverage, specs and production metrics) its primary unit. SEI tools organise data by team, developer, and *work items* (epics, initiatives, "deliverables", investment categories) taken from Jira or Linear. Portals organise it by service, component or system, with custom entity types available (Port blueprints). Codebase-understanding tools organise it by repo, file, hotspot or wiki topic. The nearest analogues are Jellyfish "Deliverables" (work-tracker based), Faros "initiative tracking", and Port's configurable data model, where a customer *could* define a "feature" blueprint by hand.

### Cited Findings
- Jellyfish's Deliverables view covers "epics, releases, or initiatives, however a team organizes its work". It is based on Jellyfish's proprietary allocation model for calculating effort, projects completion dates, and auto-categorises work by initiative, work type and business priority. It integrates with Jira, ADO and Linear, plus GitHub, GitLab, Bitbucket and ADO Repos. Its "feature" is therefore a planning-tool artefact, not a code area. — [Jellyfish Software Delivery Management](https://jellyfish.co/solutions/software-delivery-management/); [Jellyfish scenario planner blog](https://jellyfish.co/blog/software-delivery-management-scenario-planner/); [AWS Marketplace listing](https://aws.amazon.com/marketplace/pp/prodview-ma7h6oqkfhuo4)
- Faros AI sells pre-built modules for DORA metrics, developer experience, AI copilot impact, software quality, **initiative tracking** and R&D cost capitalisation. It ingests task, coding, deployment, quality, incident, security, org-structure and survey data from 100+ tools. — [Faros blog: GetDX alternatives](https://www.faros.ai/blog/getdx-alternatives)
- Port positions a "predefined data model" so ownership and workflow status stay connected to findings, and it now markets itself against Jellyfish as an engineering-intelligence product. — [Port vs Jellyfish](https://www.port.io/blog/port-vs-jellyfish-engineering-intelligence)
- Gartner defines the category (now "Developer Productivity Insight Platforms", formerly SEI) as platforms that aggregate signals from Git, issue trackers and CI/CD to measure productivity, delivery health and, increasingly, AI impact. The definition contains no feature entity. — [Gartner Peer Insights category](https://www.gartner.com/reviews/market/software-engineering-intelligence-platforms); [LinearB Gartner page](https://linearb.io/resources/gartner-dpi-market-guide)
- In Jellyfish/LinearB/Swarmia comparisons, Jellyfish is described as the tool "built for engineering leadership" (allocation, investment) and LinearB/Swarmia as team-workflow tools. All of them are organised by team or developer. — [CodePulse comparison](https://codepulsehq.com/guides/engineering-analytics-tools-comparison); [wetheflywheel comparison](https://wetheflywheel.com/en/comparisons/linearb-vs-jellyfish-vs-swarmia/)

### Inferences
- How each category uses "feature":
  - SEI vendors mean an epic or initiative in Jira. The data comes from tickets, and code is attributed to it through PR-to-ticket links.
  - Portals use service/component/system/domain (Backstage's catalog model) or arbitrary custom blueprints (Port).
  - Behavioural-code tools (CodeScene) use files, hotspots and user-configured architectural components.
  - None of them ties a code folder to its spec, decision records, release, and production metrics (PostHog/Sentry). That end-to-end chain is the gap the proposed SaaS targets.
- Port is the most direct threat on data modelling. A customer can define a "Feature" blueprint and pipe in GitHub, Jira, Sentry and PostHog data. It would require manual modelling, though, not automatic derivation from folder structure the way features-cli does.
- Jellyfish's Deliverables comes closest to "feature progress for executives". But it stops at delivery and has no design/spec input or post-release monitoring.

### Gaps
- Could not verify from primary docs whether CodeScene's "architectural components" or Backstage/Compass entity kinds include anything named "feature". The search returned nothing on CodeScene components.
- Did not confirm whether Swarmia, LinearB or DX expose an explicit feature or initiative object beyond Jira epics. Swarmia "investment balance" and LinearB "projects" are recalled from memory and unverified.
- DeepWiki/Devin Wiki, Swimm, Unblocked and Greptile organise knowledge by repo or topic. No source was found describing a feature-level entity in any of them.

## 2. Pricing (per developer per month) and typical contract sizes

### Takeaway
Self-serve SEI and code-analysis tools cluster at roughly $15–60 per developer per month. Enterprise SEI (Jellyfish, DX) is quote-only, with entry contracts around $15k–30k a year. Portals run from free (Backstage, with heavy hidden staffing cost) to $30–65 per seat per month for SaaS portals.

### Cited Findings
**Engineering intelligence**
- **LinearB:** Essentials $29/user/month (50+ devs); Enterprise $59/user/month (100+ devs); median contract about $26k a year (secondary source). A free tier for roughly 8–10 devs is reported. — [CodePulse comparison](https://codepulsehq.com/guides/engineering-analytics-tools-comparison); [Gartner review page](https://www.gartner.com/reviews/market/software-engineering-intelligence-platforms/vendor/linearb/product/linearb)
- **Swarmia:** Sources conflict.
  - One 2026 source reports a free plan under 10 devs, paid tiers from $5/dev/month, Standard at $45/dev/month, and an AI adoption/cost add-on of +$5/dev/month.
  - Older comparisons say about $15–25/dev/month.
  - — [CodePulse](https://codepulsehq.com/guides/engineering-analytics-tools-comparison); [devclocked](https://devclocked.com/posts/software-engineering-intelligence-platforms)
- **DX (Atlassian):** No public pricing. An AWS Marketplace-based comparison says it starts at $15,000 a year, plus $672 per added contributor per year (about $56/month). — [navigara: DX alternatives](https://navigara.com/blog/dx-alternatives)
- **Jellyfish:** Quote-only.
  - An AWS bundle is $30,800 a year for 15 seats, which is about $171/seat/month.
  - Another estimate is about $588 per contributor per year (about $49/month). These conflict.
  - Said not to make sense below about 200 developers.
  - — [CodePulse: Jellyfish alternatives](https://codepulsehq.com/guides/jellyfish-alternative); [meetzest](https://meetzest.com/for/jellyfish-alternative/)
- **GitKraken Insights:** About $9–19 per contributor (unverified blog figure). — [CodePulse Gartner MQ guide](https://codepulsehq.com/guides/gartner-developer-productivity-magic-quadrant)

**Codebase understanding**
- **CodeScene:** €18 (Standard) / €27 (Pro) per active author per month, billed yearly; free for open source. Blog dated 29 Oct 2026, not the vendor page. — [repowise: Git analytics tools](https://repowise.dev/blog/comparisons/best-git-analytics-tools)

**Developer portals**
- **Port:** Free up to 15 seats; Basic $30/seat/month (billed annually, up to 50 seats); Standard $40/seat/month. — [developerportalcost.com](https://developerportalcost.com/platforms); [dupple](https://dupple.com/learn/best-internal-developer-portals)
- **Roadie (managed Backstage):** Teams tier $24/dev/month, flagged "existing subscribers only". — [developerportalcost.com](https://developerportalcost.com/platforms)
- **Cortex:** Not public. Estimates cluster around $65/user/month, attributed to a July 2024 Forrester study (older info). — [developerportalcost.com](https://developerportalcost.com/platforms)
- **OpsLevel:** Custom, quote-based pricing. — [developerportalcost.com](https://developerportalcost.com/platforms)
- **Backstage (self-hosted):** Licence-free, but TCO estimates range from about $95k a year (20-dev team) to 2 FTEs / $300k a year for maintenance (both vendor-influenced). — [DEV: Backstage is not free](https://dev.to/riya_mittal_cdd264250ad45/backstage-is-not-free-the-real-tco-of-building-vs-buying-an-internal-developer-platform-3hej); [platformengineeringcost.com](https://platformengineeringcost.com/developer-portal-cost)

### Inferences
- Market price anchors for a per-feature dashboard:
  - $20–40/dev/month competes with LinearB Essentials, Port Basic/Standard and CodeScene.
  - Selling top-down to VPs (Jellyfish-style) supports $50+/dev/month, but needs 200+ dev orgs and a long sales cycle.
- A free tier for small teams (under 10–15 seats) is standard across LinearB, Swarmia and Port. A free OSS CLI fits this pattern as the bottom of the funnel.

### Gaps
- Could not verify current list prices on vendor pages (fetch blocked).
- No reliable 2026 pricing found for: Haystack, Multitudes, Waydev, Allstacks, Sleuth, Spotify Portal for Backstage, Atlassian Compass, Swimm, Greptile, CodeRabbit, Unblocked or Sourcegraph.
- No reliable source found for typical Jellyfish/DX ACVs beyond the AWS figures above.

## 3. Funding, acquisitions and shutdowns (2023–2026): what they signal about willingness to pay

### Takeaway
Consolidation is the dominant pattern. Platform owners (Atlassian, Cursor/Anysphere, Harness, GitKraken, Appfire) are buying SEI, code-review and code-context companies and folding them into suites. That signals buyers will pay for engineering data, but mostly as part of a platform they already use. Standalone point tools with weak distribution (CodeSee, Pluralsight Flow) have been absorbed or sunset.

### Cited Findings
**Engineering intelligence**
- **DX → Atlassian (confirmed):**
  - Announced September 2025 for about US$1B in cash plus restricted stock. Atlassian's largest acquisition to date.
  - "Nearly all" of DX's customers were already Atlassian customers.
  - DX is being folded into Atlassian's "system of work" alongside Rovo Dev, Jira and Bitbucket.
  - Expected to close in Q2 of Atlassian FY2026.
  - Some outlets quote "$1.5B", which appears to be the A$ figure.
  - — [BNN Bloomberg/Reuters](https://www.bnnbloomberg.ca/business/2025/09/18/atlassian-to-acquire-developer-productivity-platform-dx-for-us1-billion/); [Constellation Research](https://www.constellationr.com/insights/news/atlassian-buys-dx-1-billion); [Capital Brief](https://www.capitalbrief.com/briefing/atlassian-buys-us-software-firm-dx-in-15b-deal-29ea1852-c47c-4d3b-b8b4-1f9cb6f444b8/)
- **Gartner:**
  - Market Guide for Developer Productivity Insight Platforms dated 12 May 2025; LinearB named a Representative Vendor. — [LinearB](https://linearb.io/resources/gartner-dpi-market-guide)
  - A first Magic Quadrant (May 2026), naming Jellyfish, Atlassian (via DX), LinearB and Opsera as Leaders, is reported by third parties only (unverified). — [CodePulse](https://codepulsehq.com/guides/gartner-developer-productivity-magic-quadrant); [Owler LinearB](https://www.owler.com/company/linear-b)
  - Gartner reportedly predicts 50% of engineering orgs will use such platforms by 2027 (forum post citing Gartner). — [10x.pub forum](https://tianpan.co/forum/t/gartner-50-of-engineering-orgs-will-use-intelligence-platforms-by-2027-the-10x-adoption-wave-is-here/311)
- **Pluralsight Flow → Appfire:**
  - Acquired 5 Feb 2025. Flow was GitPrime, bought by Pluralsight in 2019 for $170M. — [Appfire newsroom](https://appfire.com/newsroom/appfire-acquires-flow); [DevOps Digest](https://www.devopsdigest.com/appfire-acquires-flow)
  - Competitor blogs say Appfire has since scheduled Flow's end of life (December 2027 date cited). Not verified against an Appfire notice. — [Larridin](https://larridin.com/blog/why-appfire-flow-shut-down); [Waydev](https://waydev.co/gitprime-vs-waydev-vs-code-climate/)
- **Jellyfish:** Total funding about $114.5M ($12M Series A 2020; $31.5M Series B 2021 led by Insight Partners; the remainder is presumably the 2022 Series C, not itemised in the source). — [Sacra](https://sacra.com/c/jellyfish/)
- **LinearB:** Total funding reported variously as $70M, $71M, "over $80M" or $84.1M (PitchBook via Waydev). Unresolved. — [Owler](https://www.owler.com/company/linear-b); [Cortex blog](https://www.cortex.io/post/engineering-intelligence-platforms-definition-benefits-tools)
- **Swarmia:** Funding conflicts: $19.7M over 3 rounds (Tracxn) vs $12M Series A in June 2025 and $13M total (StartupIntros). — [Tracxn](https://tracxn.com/d/companies/swarmia/__3EkVJl86yqUhSmHPIcHL6I3AXtQ7a0X0IqyLjUch0vc); [StartupIntros](https://startupintros.com/orgs/swarmia)
- **Faros AI:** Conflicting figures.
  - $36M total, latest a $20M Series A in June 2023 (CB Insights).
  - $64.8M including a $25M Series B in June 2026 (PitchBook).
  - Another source says no Series B was announced.
  - — [CB Insights](https://www.cbinsights.com/company/faros); [PitchBook](https://pitchbook.com/profiles/company/435439-81); [rywalker](https://rywalker.com/research/faros-ai)
- **Waydev:** Says it has never taken institutional capital beyond Y Combinator. Rebuilt the product around AI-impact measurement (July 2026 post). — [Waydev: Nine hard years](https://waydev.co/why-we-rebuilt-waydev-for-the-ai-era/)

**Developer portals**
- **Port:** $100M Series C (PitchBook dates it October 2025; another source says December 2025 at an $800M valuation); total funding about $160M. — [PitchBook Port.io](https://pitchbook.com/profiles/company/489602-98); [tasrieit](https://tasrieit.com/blog/port-vs-backstage-vs-cortex-developer-portal-comparison-2026)
- **Cortex:** Only a Tracxn table entry (Series C; "$112M" figure, unclear whether valuation or funding). Unverified. — [Tracxn via Swarmia profile](https://tracxn.com/d/companies/swarmia/__3EkVJl86yqUhSmHPIcHL6I3AXtQ7a0X0IqyLjUch0vc)

**Codebase understanding and AI code review**
- **CodeSee → GitKraken:**
  - CodeSee announced it would close and stop commercial operations on Thursday 22 Feb 2024, per a LinkedIn post by a team member.
  - GitKraken announced the acquisition on 14 May 2024 and folded CodeSee's code-visualisation tech into its new "DevEx" platform.
  - No official statement of the reason for the shutdown was found.
  - — [SD Times](https://sdtimes.com/software-development/gitkraken-acquires-codesee-launches-new-devex-platform/); [AlternativeTo](https://alternativeto.net/news/2024/5/gitkraken-acquires-codesee-and-launches-new-developer-experience-platform-named-devex); [LinkedIn post](https://sk.linkedin.com/in/daniel-ivanyi-16845458)
- **Graphite → Cursor (Anysphere):**
  - Announced 19 December 2025. Price undisclosed but above Graphite's last $290M valuation (Axios via secondary sources). Graphite had raised about $72M.
  - Cursor said it would keep Graphite as a standalone product.
  - Unverified: a Wikipedia snippet says Cursor itself became a SpaceX subsidiary in 2026 (a $60B all-stock deal is reported by aggregators).
  - — [Contrary Research](https://research.contrary.com/company/graphite); [Jesterwise](https://www.jesterwise.com/2025/12/cursor-continues-acquisition-spree-with.html); [Wikipedia: Cursor](https://en.wikipedia.org/wiki/Cursor_(company))
- **CodeRabbit:** About $87.6M raised at a $550M valuation (as of January 2026, secondary source). — [implicator.ai](https://www.implicator.ai/cursors-29-billion-bet-on-a-bottleneck-it-may-be-creating/)
- **Greptile:** $25M Series A (2025). A valuation of $85M is unconfirmed. — [implicator.ai](https://www.implicator.ai/cursors-29-billion-bet-on-a-bottleneck-it-may-be-creating/)
- **Sourcegraph / Amp:**
  - Amp spun out of Sourcegraph as an independent company on 2 Dec 2025, run by co-founders Quinn Slack and Beyang Liu. Dan Adler leads the remaining Sourcegraph code-search business.
  - Amp Free (ad-supported) reportedly reached a $10M+ run rate before ads were dropped in March 2026 (single secondary source).
  - — [Wikipedia: Sourcegraph](https://en.wikipedia.org/wiki/Sourcegraph); [rywalker: Amp](https://rywalker.com/research/amp-sourcegraph); [GitHub issue noting spinout](https://github.com/benjamincanac/whichcodingtools/issues/80)
- **Augment Code:**
  - Raised about $252M ($25M Series A plus $227M Series B led by Sutter Hill), at about a $977M valuation ($1.4B reported by one outlet).
  - Unbundled its Context Engine as an MCP server, GA 6 Feb 2026.
  - **Harness acquired select Augment assets** (Cosmos agent system, Auggie CLI, Context Engine and teams) in roughly Q3–Q4 2026; terms undisclosed.
  - — [Dealroom: Harness acquires Augment](https://dealroom.co/news/160997-harness-acquires-augment-code-to-automate-software-from-idea-to-producti/); [remio.ai](https://www.remio.ai/post/harness-augment-code-acquisition-connects-ai-coding-to-production); [rywalker: Augment Context Engine](https://rywalker.com/research/augment-context-engine)

### Inferences
- Willingness to pay is real but is captured by suites:
  - Atlassian paid about $1B for DX, whose customers were almost all Atlassian customers. That suggests the value was in bundling metrics into Jira/Bitbucket, not standalone distribution.
  - Cursor buying Graphite and Harness buying Augment's context engine show "idea → code → production" consolidation. That is the same end-to-end narrative the proposed SaaS pitches, now being pursued by much better-funded players.
- The SEI pitch has shifted toward "AI impact measurement" (DX, Waydev rebuild, Swarmia AI add-on, Faros copilot module). A per-feature dashboard can ride this wave by showing per-feature AI-generated code share, defects and cost, which no one currently frames per feature.
- Shutdowns and sunsets (CodeSee, Flow) suggest that code visualisation alone, and SEI without a platform owner, struggle to monetise. This is a warning for a visualisation-led feature map.

### Gaps
- Not covered (no results found within the search budget):
  - 2023–2026 funding or news for Haystack, Multitudes, Allstacks, Sleuth, OpsLevel, Roadie, Swimm, Unblocked and Cognition/DeepWiki.
  - Atlassian Compass status (whether discontinued or folded into another product).
  - Spotify Portal for Backstage pricing and GA status.
- Unverified leads to check:
  - Port Series C valuation ($800M) and exact date.
  - Faros Series B (June 2026).
  - The Gartner MQ 2026 Leaders list.
  - The Appfire Flow EOL date.
  - The DX deal's closing date.
- Customer counts and ARR for Jellyfish, LinearB, Swarmia, Port and Cortex were not found.

## 4. Which features overlap directly with features-cli (ownership, per-area debt and coverage, docs for AI agents)?

### Takeaway
Each features-cli capability has a strong incumbent, but no single incumbent combines them, and none keys them to feature folders:
- Ownership and catalog: Backstage, Port, Cortex, OpsLevel.
- Per-area debt and knowledge risk: CodeScene.
- Delivery and git metrics: LinearB, Swarmia, Jellyfish.
- AI-agent context (wiki/MCP): Augment Context Engine, DeepWiki, Sourcegraph, Unblocked.
The overlap risk is highest with Port (custom data model plus scorecards plus engineering intelligence) and CodeScene (hotspots, ownership, debt by component).

### Cited Findings
- Portals own the "ownership" and "scorecard" space. Port says ownership and workflow status remain connected to findings in its data model, and it competes directly with Jellyfish on engineering intelligence. — [Port vs Jellyfish](https://www.port.io/blog/port-vs-jellyfish-engineering-intelligence)
- Backstage-based portals are the default catalog. Backstage is said to hold about 89% market share among IDPs (forum claim), and two-thirds of platform teams run it (vendor blog). — [10x.pub](https://tianpan.co/forum/t/backstage-holds-89-market-share-but-only-10-internal-adoption-why/417); [bex.co](https://bex.co/blog/2026/09/23/backstage-golden-path-idp-self-host-vs-port-cortex-cost)
- SEI platforms cover git history and delivery metrics.
  - Faros covers task, coding, deployment, quality, incident, security and org data from 100+ tools. — [Faros](https://www.faros.ai/blog/getdx-alternatives)
  - Jellyfish categorises work by initiative and links to Git and issue trackers. — [Jellyfish](https://jellyfish.co/solutions/software-delivery-management/)
- Docs and context for AI agents: Augment unbundled its codebase "Context Engine" as an MCP server any agent can call (GA Feb 2026). This is a direct analogue to the features-cli MCP server, though organised by semantic index, not features. — [rywalker: Augment Context Engine](https://rywalker.com/research/augment-context-engine)
- CodeScene sells per-author code health analysis (Standard/Pro tiers). — [repowise](https://repowise.dev/blog/comparisons/best-git-analytics-tools)

### Inferences
| features-cli capability | Closest incumbents | Unit they use |
|---|---|---|
| Owners / CODEOWNERS generation | Backstage, Port, Cortex, OpsLevel (catalog ownership); CodeScene (knowledge maps) | service/component; file/team |
| Test coverage per area | Cortex/OpsLevel/Port scorecards (via integrations) | service |
| Tech debt per area | CodeScene (hotspots, code health) | file/component |
| Git history per area | LinearB, Swarmia, Jellyfish, Faros | team/developer/epic |
| Specs and decision records | Swimm (docs coupled to code), Backstage TechDocs | repo/doc |
| MCP server for AI agents | Augment Context Engine, Sourcegraph, DeepWiki, Unblocked | repo/semantic index |
| PostHog/Sentry metrics per area | Port/Cortex via integrations (manual mapping) | service |

- Differentiator: features-cli derives the "feature" unit automatically from folder structure. That works for monoliths and monorepos, where service-centric portals fit poorly because one service contains dozens of features. This is a plausible wedge.
- Caveat: the table is an inference from category descriptions. Exact per-vendor capabilities (e.g. CodeScene's MCP server, Cortex Sentry integration) were not verified in this session.

### Gaps
- Not verified: whether CodeScene ships an MCP server or AI-agent integration; whether Swimm still operates and at what price; Unblocked's and DeepWiki's feature sets and pricing; and whether Sourcegraph offers per-directory ownership analytics.

## 5. What criticism exists of SEI tools and of portals?

### Takeaway
- SEI tools are criticised for surveillance risk (especially individual-level dashboards), gameable or vanity metrics, and poor fit for small orgs.
- Backstage is criticised for heavy setup and maintenance burden and low internal adoption (about 10% is a widely repeated but poorly sourced figure).
- Most of the criticism found is published by competitors, so it is directional.

### Cited Findings
**SEI tools**
- 66% of developers do not believe, or are unsure, that current metrics reflect their real contributions (JetBrains 2025 survey, reported secondhand). — [Macroscope: without surveillance](https://macroscope.com/content/measure-engineering-productivity-without-surveillance)
- Surveillance concerns appear more often in LinearB discussions. A competitor calls LinearB's individual coaching dashboards a "red flag". — [CodePulse](https://codepulsehq.com/guides/engineering-analytics-tools-comparison); [GitKraken Insights vs LinearB](https://gitkraken.com/insights/competitors/insights-vs-linearb)
- When engineers perceive tools as surveillance, survey responses become dishonest and commit data gets gamed (implementation-guide claim, no study cited). — [Macroscope](https://macroscope.com/content/engineering-productivity-analytics)
- Jellyfish is criticised for not making sense below about 200 developers and for lacking a public API for metric extraction. — [CodePulse: Jellyfish alternatives](https://codepulsehq.com/guides/jellyfish-alternative)
- Peer Insights sample sizes are small: Jellyfish 64 reviews (4.7), LinearB 4 reviews (4.2) on the Gartner DPI comparison page. — [Gartner Peer Insights](https://gcom.pdo.aws.gartner.com/reviews/market/developer-productivity-insight-platforms/compare/jellyfish-vs-linearb)

**Portals**
- Backstage internal adoption reportedly "hovers around 10%", often because teams exhaust capacity on maintenance (vendor-run site; no primary source). — [10x.pub forum](https://tianpan.co/forum/t/backstage-holds-89-market-share-but-only-10-internal-adoption-why/417)
- Anecdotes:
  - A platform team spending 70% of its time on Backstage maintenance vs 30% on DX improvements.
  - A 150-engineer client reaching 12% adoption after 8 months, with 60% of time spent on maintenance.
  - Spotify itself ran a 4-engineer full-time IDP team for about 1,600 engineers.
  - — [10x.pub: "Backstage team"](https://tianpan.co/forum/t/our-platform-team-became-the-backstage-team-were-maintaining-a-product-not-building-platform-capabilities/2941); [DEV: Backstage TCO](https://dev.to/riya_mittal_cdd264250ad45/backstage-is-not-free-the-real-tco-of-building-vs-buying-an-internal-developer-platform-3hej)
- A balanced view: Backstage genuinely pays off for companies with 20+ services, rarely for smaller teams. — [DEV: Backstage alternatives 2026](https://dev.to/danielwe/backstage-alternatives-in-2026-first-ask-why-you-wanted-backstage-1iae); [kubewright: Backstage is not your platform](https://kubewright.co.uk/blog/backstage-is-not-your-platform/)

### Inferences
- Positioning lessons for a per-feature dashboard:
  - Avoid individual-developer metrics. Aggregating by feature rather than by person sidesteps the surveillance critique and is a selling point.
  - Zero-config derivation from the repo (folder → feature), plus CODEOWNERS generation, directly answers the portal criticism of manual catalog upkeep. A catalog that goes stale is the main adoption killer.
  - "Metrics without context" is the core SEI complaint. Linking each metric to a feature's spec and outcome (PostHog/Sentry) offers context SEI tools lack.

### Gaps
- No neutral or peer-reviewed study found on Backstage adoption rates or on SEI surveillance effects.
- Did not retrieve Hacker News or Reddit threads directly (search budget). G2 review themes were not gathered.
- Criticism of Port/Cortex/OpsLevel (e.g. data-model setup effort, pricing at scale) was not found in this session.
