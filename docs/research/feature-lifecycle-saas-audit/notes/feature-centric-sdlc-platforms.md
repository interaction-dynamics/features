# Feature-centric SDLC platforms: does anything organise the whole lifecycle by feature? (as of Nov 2026)

Method note: research done 2026-11-19 through web search, with a few page fetches. Many vendor hosts could not be reached from this session and failed DNS: atono.io, atono.substack.com, tggl.io, www.ycombinator.com. The GitHub API was also blocked for unattached repos. Findings on those products therefore rely on search-result snippets and third-party listings, as noted in each entry. The YC directory could not be queried directly, so YC coverage relies on batch round-ups and is incomplete. Products already covered in sibling notes (Linear, Jira/JPD/Rovo, Productboard, Aha!, Jellyfish, Port, Backstage, LaunchDarkly, PostHog, Datadog, CodeSee, Height, Kiro, Spec Kit, Tessl, Augment, Jama, Polarion) appear here only where new 2026 facts change the picture.

## Key question: Does a product that organises SDLC by feature exist today? Closest matches ranked, and what each is missing

### Takeaway
No. As of Nov 2026 I found no active commercial or open-source product that makes a code-level "feature" the primary entity and attaches design, spec, tasks, PRs, tests, release/flags and production usage/errors to that one record. The attempt to falsify the earlier finding failed. Each candidate covers one end of the lifecycle:
- **Planning-to-release:** Atono, Azure DevOps "Feature", Linear with Diffs/Releases.
- **Flag-to-adoption:** Reflag (ex-Bucket), Unleash lifecycle.
- **Spec-to-code:** Kiro, OpenSpec, dspec.
- **Code-to-feature map:** ProductMap AI, plus a new crop of small `features.yaml` open-source tools.

None of them joins all four. The newest signal is grassroots: since 2025–2026, many tiny open-source projects declare features in YAML inside the repo (`.features/*.yaml`, `FEATURE-MAP.yaml`, `.codebase/features/*.yaml`, `.ds/`). All have zero or few stars. Developers are hand-building the missing entity for AI coding agents, and no vendor has productised it.

### Closest matches, ranked (most complete lifecycle coverage first)

| # | Product | Primary entity | Stages covered | Links to code? | Missing |
|---|---|---|---|---|---|
| 1 | **Atono** (startup, Toronto per Capterra) | Story, with feature flags attached to the story | Plan → build → flag rollout → (claims) measure | GitHub integration (PR to story, per directory listings); flags via OpenFeature SDKs | Unit is the story, not a durable feature. No error or production telemetry per feature found. No spec or design artifact. Feature stops existing once its stories close |
| 2 | **Azure DevOps Boards "Feature" work item** (Microsoft) | Epic > **Feature** > User Story/PBI (process template) | Plan → branch/commit/PR/build (Development control) → release stages (Deployment control) → test plans | Yes: branches, commits, PRs, builds, releases linked to work items | Feature is a planning container. Code links attach to child items, and the Feature carries no code-level identity (no module or path ownership). No production usage or error data. No living spec |
| 3 | **Reflag** (formerly Bucket.co) | **Feature** (feature key) | Release stages (In development → Internal → Beta → GA) → flag targeting → adoption tracking → feedback | Feature key in code via SDK/CLI; MCP/agent tooling | No spec, design, tasks or PRs. No error tracking. Lifecycle begins at "flag created", not at idea |
| 4 | **Linear (2026: Diffs + Releases + release-pipeline changelogs)** | Issue/Project | Plan → native code review (Diffs, May 2026) → release pipelines/notes (May–Jun 2026) | Yes, via GitHub integration and Diffs | Project ≠ feature. No flags, no production usage or errors. Projects end when work ships, so nothing stays as a living feature record |
| 5 | **Planview Viz (Flow Framework)** | Flow item of type **Feature** (also Defect, Risk, Debt) | Measures flow concept → cash across connected tools | Indirect, via tool integrations (Planview Hub) | Analytics overlay only, with no artifacts held. Feature = work-item type, not a code entity |
| 6 | **Kiro / OpenSpec / dspec** (spec-driven tools) | Per-feature spec folder (`requirements.md`, `design.md`, `tasks.md`) or per-change folder | Spec → design → tasks → code | Yes: lives in the repo. dspec fingerprints code symbols (`codeRef`) | No release, flag, usage or error stages. No PM/team UI |
| 7 | **ProductMap AI** (Product Hunt, May 2025) | Feature tree inferred from code | Code understanding only | Yes: features mapped with traceability to code | No planning, release or production stages. Traction unclear |
| 8 | **Serenity BDD living documentation** (open source) | Capability > **Feature** > Story (test-folder hierarchy) | Spec (Gherkin) ↔ test results | Tests only | No PRs, releases, flags or usage. Test-reporting tool |
| 9 | **Kosli** | Flow / Trail (trail ID can be a ticket or PR) | Commit → build → attestations → which commit runs in prod | Yes: artifact fingerprints to environments | Change/compliance-centric, not feature-centric |

### Cited Findings

**Atono (active, private startup)**
- Atono puts feature flags directly into stories: "feature flags are integrated directly into stories, so you can toggle features on or off right from the stories you're working on." The changelog also says it can "directly link feature flags to stories, with integrated documentation and code snippets" — [Atono docs: Feature flags](https://docs.atono.io/docs/feature-flags); [Atono changelog](https://docs.atono.io/changelog/feature-flags)
- Its flags are based on openfeature.dev. Once a flag is added to a story, teams must integrate it into code. Reporting shows "all stories under flag control" to help with flag cleanup — [Atono docs](https://docs.atono.io/docs/feature-flags); [Flag status](https://docs.atono.io/docs/flag-status)
- Atono calls itself a "product engineering platform" for teams building with AI-assisted tools. Third-party listings describe it as spanning plan, build, deploy and measure in one system. Snippets mention integrations with Slack, GitHub and an Atono MCP Server, plus imports from Jira/Linear — [Crunchbase](https://crunchbase.com/organization/atono); [Webrazzi, Dec 2025](https://webrazzi.com/2025/12/26/urun-ekipleri-icin-yol-haritasindan-yayina-tek-akista-urun-gelistirme-platfomu-atono/); [topai.tools](https://topai.tools/t/atono)
- One directory says the GitHub integration "automatically links pull requests to relevant stories". The page looks auto-generated, so this is low-confidence — [whichaitools](https://whichaitools.pages.dev/tools/atono)
- Pricing has a free tier, then about $19 (Starter) and $39 (Growth) per user per month. Listings conflict on free-tier size (25 users vs 5 users; the 5-user listing is dated Jun 2025) — [Capterra India](https://www.capterra.in/software/1111273/atono); [xpay, May 2026](https://www.xpay.sh/saas-pricing/atono-io/); [SoftwareSuggest](https://www.softwaresuggest.com/atono)
- Toronto address (5000 Yonge St) per Capterra Canada. No funding data found — [Capterra CA](https://www.capterra.ca/software/1111273/atono)
- The company blog debates whether "user stories are obsolete" (title only seen), which suggests it is rethinking the unit of work — [Atono Substack](https://atono.substack.com/p/are-user-stories-obsolete)

**Reflag, formerly Bucket.co (active)**
- Bucket rebranded to Reflag; the company blog dates the announcement Sep 4, 2025. Focus: "the best feature flagging for SaaS". One third-party directory says 2026, which conflicts with the company's own date — [Reflag blog](https://reflag.com/blog/bucket-reflag); [announcements index](https://reflag.com/category/announcement)
- Positioning: "feature flagging and entitlement platform purpose-built for B2B SaaS" with company-level targeting and feature adoption metrics. It earlier marketed itself as "customer-centric feature management" — [Reflag](https://reflag.com/feature-flags); [Bucket blog](https://bucket.co/blog/introducing-customer-centric-feature-management)
- Features carry release stages: new apps start with 4 defaults (In development, Internal, Beta, General availability), customisable, with optional access rules. The same feature key drives adoption tracking and qualitative feedback — [Reflag docs: feature rollouts](https://docs.reflag.com/product-handbook/feature-rollouts)
- Agent-oriented features ("agentic feature flagging") and an MCP server exist — [Reflag agents](https://reflag.com/agents); [Glama MCP listing](https://glama.ai/mcp/servers/h3p2ndqmat)
- Funding: Seed led by Project A with Creandum participating brought total funding to $5.7M. The round date was not found — [Reflag/Bucket announcement via search](https://reflag.com/blog/bucket-reflag)

**Azure DevOps Boards (Microsoft, active)**
- The Development control on a work item form shows linked branches, commits, PRs and builds. The Deployment control shows the release stages the work item has reached. Microsoft says linking work items to builds and releases provides "an audit trail of the feature development and deployment" — [MS Learn: end-to-end traceability](https://learn.microsoft.com/en-in/azure/devops/cross-service/end-to-end-traceability?view=azure-devops); [MS Learn: connect work items to Git](https://learn.microsoft.com/en-us/azure/devops/boards/backlogs/connect-work-items-to-git-dev-ops)
- GitHub commits and PRs can be linked with `AB#` mentions. A community Q&A reports inconsistent display — [MS Learn: link GitHub](https://learn.microsoft.com/en-us/azure/devops/boards/github/link-to-from-github); [MS Q&A](https://learn.microsoft.com/en-in/answers/questions/5912402/github-pull-requests-and-commits-not-appearing-in)
- (Process-template fact, not re-verified in this session: Agile/Scrum templates use Epic > Feature > Story/PBI. SAFe tools such as Jira Align, Rally and Targetprocess similarly have a "Feature" portfolio item. All are planning constructs.)

**Linear (2026 updates, active)**
- Linear Diffs launched May 27, 2026: "review diffs from any issue with a PR, iterate on further changes with agents, and ship code from Linear". Reviews sync back to GitHub — [Linear changelog](https://linear.app/changelog/2026-05-27-linear-diffs)
- Release pipelines got per-pipeline changelogs on Jun 18, 2026. Linear Releases (auto release notes, CI/CD integration) appeared around early May 2026, per aggregated tweets — [Linear changelog](https://linear.app/changelog/2026-06-18-agent-assisted-project-updates); [releases.sh](https://releases.sh/linear/linear.md); [plushcap](https://www.plushcap.com/companies/linear/xtwitter)
- Linear's public MCP now exposes diffs, releases, release pipelines and release notes (observed from this session's own Linear connector tool list, not a web source).

**Planview Viz / Flow Framework (active)**
- The Flow Framework has four flow item types: Features (business value), Defects, Risks, Debts. Flow Metrics measure the software value stream "from concept to cash" — [Planview Flow Framework](https://www.planview.com/?p=30087); [Planview Viz](https://www.planview.com/products-solutions/products/viz/)
- Planview Viz added a "Flow Item Analyzer" in Sep 2024 — [Planview release notes](https://success.planview.com/Planview_Viz/Planview_Viz_Product_Releases/2024_Releases/09_September_2024_Release)

**Kosli (active, Series A with Deutsche Bank as investor)**
- Model: a Flow (repeatable process) runs as Trails, which produce Artifacts and collect Attestations. Trail IDs are often a commit SHA, PR number or ticket ID. Artifacts are fingerprint-matched to environment snapshots, which shows what is running in production — [Kosli docs: how Kosli works](https://docs.kosli.com/understand_kosli/how_kosli_works); [Kosli concepts](https://docs.kosli.com/understand_kosli/concepts/)
- The Series A, with Deutsche Bank as an investor, is mentioned in the site banner — [kosli.com/flows](https://www.kosli.com/flows)

**Tggl (active, French feature-flag startup)**
- Has a dedicated "Feature lifecycle management" page. Content could not be fetched (host blocked) — [tggl.io/feature-lifecycle-management](https://tggl.io/feature-lifecycle-management)
- Unleash similarly uses "feature lifecycle management" language: native lifecycle stages, stale-flag notifications, Jira/Linear integrations — [Unleash blog](https://www.getunleash.io/blog/feature-lifecycle-management)
- Optimizely frames flags as decoupling "feature lifecycle management from code deployment" — [Optimizely glossary](https://www.optimizely.com/optimization-glossary/feature-management/)

**Spec-driven / features-as-code tools**
- Kiro writes `requirements.md`, `design.md` and `tasks.md` per feature. OpenSpec gives each change its own folder (proposal, specs, design, tasks). Augment Cosmos (reported May 2026 launch) is a cloud platform where agents draft specs, engineers review intent, and parallel agents implement — [Augment: best SDD tools 2026](https://www.augmentcode.com/tools/best-spec-driven-development-tools); [OpenSpec GitHub](https://github.com/Fission-AI/OpenSpec)
- dspec (tuna781/dspec) is a Claude Code plugin that models the repo in `.ds/` as "the source of truth". `/ds:sync` compares spec and code in both directions, and each `codeRef` fingerprints the symbol it names, "so drift is measured rather than remembered" — [github.com/tuna781/dspec](https://github.com/tuna781/dspec)
- Factory (droids) offers a Specification Mode with an approval gate and saves the approved design as a Markdown spec in git. Devin uses Knowledge and Playbooks. Factory reportedly raised a $150M Series C in Apr 2026 (third-party comparison site; unverified) — [andrew.ooo comparison, Aug 2026](https://andrew.ooo/answers/warp-factories-vs-factory-ai-vs-devin-august-2026/); [aicentralresources](https://www.aicentralresources.com/compare/factory-vs-devin)
- GitHub Copilot Workspace was sunset around May 30, 2025 (one source says Nov 2025, which conflicts). Its issue-to-PR model was rebuilt as the Copilot coding agent. Agent HQ / Mission Control (announced Oct 2025) is a command centre for "assigning, steering, and tracking" multiple agents. I found no evidence it persists a feature-level record — [GitHub blog: Agent HQ](https://github.blog/news-insights/company-news/welcome-home-agents/); [The New Stack](https://thenewstack.io/github-agent-hq/); [Java Code Geeks (conflicting date)](https://www.javacodegeeks.com/?p=141630)

**Code-to-feature mapping (commercial)**
- ProductMap AI launched on Product Hunt on May 8, 2025 (#11 of the day, 175 upvotes). It uses AI to identify "the features implemented in the code and shows them on a map with their hierarchy and traceability to the code" ("Google Maps for code"). The maker said it handled repos up to 300k LoC and targeted 1M. No pricing or funding found — [Product Hunt](https://www.producthunt.com/products/productmap-2); [hunted.space](https://hunted.space/dashboard/productmap-2/launches/productmap-ai); [chatgate.ai](https://chatgate.ai/post/productmap-ai)

**Grassroots open-source "feature registry in the repo" (2025–2026)**
- Amvelt TRACE (anubhav-n-mishra/ABS-Trace) maps a repo "by features, capabilities, and system boundaries". It infers features from routes, directories and AST with confidence scores, or reads explicit `.codebase/features/*.yaml` listing components, services, routes, models and tests. It gives agents context slices and CI rule checks. It has 0 stars and 13 commits, and no PR or spec linking — [GitHub](https://github.com/anubhav-n-mishra/ABS-Trace)
- ade-syofyan/feature-map keeps a `FEATURE-MAP.yaml` registry of business flows across client, backend, admin and docs. It audits drift as GAP/DRIFT/IMPACT/OK and ships as a Claude Code plugin and Codex skill — [GitHub](https://github.com/ade-syofyan/feature-map)
- markschellhas/feature-map keeps `.features/*.yaml`, with a CLI to list, search, validate (checks referenced paths exist) and graph them — [GitHub](https://github.com/markschellhas/feature-map)
- dzanfardino/feature-registry is a "domain-agnostic catalog for software features" (NestJS/React/Postgres). It tracks features, flags (with `expectedSunsetAt` reminders), cross-team dependencies, pre-release checklists and Jira/Confluence links. It has no code-repo linking and 0 stars — [GitHub](https://github.com/dzanfardino/feature-registry)
- Issue proposals exist for the same pattern: g8s #649 proposes a `.g8s/feature-map.yml` DAG of user stories, code paths and tests, with a CI check on dangling references. chatstrata #45 proposes a "load-bearing" feature map with drift gates — [g8s #649](https://github.com/tamld/g8s/issues/649); [chatstrata #45](https://github.com/brandonbosch/chatstrata/issues/45)
- A PiwiTests PR adds a feature catalog (`piwi-features.ts`) that generates a docs feature map, so "the map can never drift from the catalog" — [PiwiTests PR #499](https://github.com/PiwiTests/platform/pull/499)
- Codex (OpenAI CLI) keeps an internal centralized feature registry with lifecycle metadata (a static `FEATURES` constant) for its own experimental features — [DeepWiki](https://deepwiki.com/halfprice06/codex/6.2-feature-flags-and-tool-specs)
- Research context: an arXiv study found that in open-source GitHub projects "a large portion of documented features did not offer linked implementation, indicating lacking traceability" — [arXiv 2208.01317](https://arxiv.org/pdf/2208.01317)

**BDD / living documentation**
- Serenity BDD organises requirements as capabilities > features > stories (folders under `src/test/resources/features`) and renders a Requirements tab with test results per level — [Serenity docs: living documentation](https://serenity-bdd.github.io/docs/reporting/living_documentation); [annotation requirements](https://serenity-bdd.github.io/docs/guide/annotation-requirements)
- CucumberStudio (SmartBear) is still operating: a domain-change FAQ was updated Oct 30, 2025, and I found no EOL notice. SpecFlow was abandoned by Tricentis. Cucumber's own decline is debated in vendor and opinion blogs — [SmartBear FAQ](https://support.smartbear.com/cucumberstudio/docs/faq.html); [Agile Way: SpecFlow dying](https://agileway.substack.com/p/specflow-is-dying-another-prediction); [testRigor (vendor opinion)](https://testrigor.com/blog/why-cucumber-and-specflow-died/)

**Product analytics "feature" concepts (UI-side definitions)**
- Pendo lets teams "tag a page or feature anytime" with the visual designer, and backfills data to install. A feature there is a UI element or click target defined after the fact, not a code entity. Pendo's 2019 report claimed $29.5B in cloud R&D was "squandered" on unused features — [Pendo vs Amplitude](https://www.pendo.io/pendo-vs-amplitude/); [Pendo PR](https://prnewswire.com/news-releases/pendo-data-suggests-29-5-billion-in-global-cloud-rd-investment-squandered-software-features-go-unused-300789477.html)
- Amplitude and Mixpanel are event-based and need instrumentation. Here "feature adoption" is an analysis over named events, with no feature object — [TrustRadius comparison](https://www.trustradius.com/compare-products/amplitude-analytics-vs-mixpanel-vs-pendo); [Userpilot](https://userpilot.com/blog/pendo-vs-amplitude/)
- LaunchDarkly's own argument: monitoring tools track app and infra health, "but not feature behavior". A feature management platform should trace errors and regressions back to the features active at the time — [LaunchDarkly blog](https://launchdarkly.com/blog/core-capabilities-enterprise-feature-management-platform/)

**Dead or retiring products relevant to the feature-as-unit idea**
- Delibr, a Stockholm "feature refinement" document tool for PMs with Jira epic/story sync, announced a full shutdown on Aug 31, 2026, so it is now dead. Its epic template had the headings Problem / Solution / User stories / Rollout — [delibr.com](https://www.delibr.com); [Delibr help: epic documents](https://help.delibr.com/article/128-epic-documents)
- Pivotal Tracker reached end of life on Apr 30, 2025 (announced Sep 2024) — [Pivotal Tracker EOL](https://pivotaltracker.com/blog/2024-09-18-end-of-life)
- (Already known: CodeSee and Height shut down.)

**YC / funding scan**
- The YC 2025 batches round-ups surfaced no company explicitly doing feature lifecycle or spec-to-production tracking. Adjacent companies: Interfere (S25, "self-healing production visibility"), Stagewise, Corelayer (AI on-call). Software-dev tools were the largest category in S25 — [CB Insights S25](https://www.cbinsights.com/research/y-combinator-summer2025/); [Forbes F25](https://www.forbes.com/sites/dariashunina/2025/11/13/the-top-startups-to-watch-from-y-combinators-fall-2025-batch/)
- Recent 2026 dev-tool funding is concentrated in verification (&Truth, $70M seed), agent governance (Asymptote Labs $5.5M, AIR $50M), agent testing (Plurai $13M) and coding agents (Kilo Code $8M). None is positioned as a feature system of record — [Unite.AI](https://www.unite.ai/and-truth-raises-70m-seed-to-build-universal-verification-machine-for-code/); [TechCrunch AIR](https://techcrunch.com/2026/09/01/air-raises-50m-to-help-companies-vet-the-skills-and-add-ons-ai-agents-use/); [AlleyWatch](https://alleywatch.com/2026/10/asymptote-labs-ai-agent-governance-runtime-infrastructure-security-beacon-open-source-telemetry-justin-d-souza/)
- Searches for "feature inventory" and "feature portfolio" SaaS returned only IT SaaS-app inventories (Reco, CloudNuro) and deprecation-analysis how-tos (Bruin, SaasDash). No product tracks a company's own features with owner, usage and code — [Bruin](https://getbruin.com/use-cases/saas/feature-deprecation-impact/); [SaasDash](https://saasdash.ai/blog/saas-when-to-sunset-a-feature)

### Inferences
- **The feature-centric SDLC slot is still empty, but the edges are moving toward it fast.** In 2026 Linear added code review and releases, Atono merged stories with flags, Reflag attaches release stages and adoption to a feature key, and spec tools persist per-feature specs in repos. A product that joins Reflag-style feature keys and stages to Kiro-style in-repo specs and Azure DevOps-style dev/deploy links would be new. Each piece exists separately.
- **"Feature" means different things in each camp, and that is the core reason nobody unifies it:**
  - Planning tools: a work-item container (Azure DevOps Feature, SAFe Feature, Linear Project).
  - Flag tools: a runtime switch (Reflag, Unleash, LaunchDarkly).
  - Analytics: a UI click target (Pendo) or an event set (Amplitude).
  - Spec tools: a folder of Markdown (Kiro).
  - Code mappers: an inferred cluster of files (ProductMap, TRACE).
  A durable feature ID shared across these is the missing primitive.
- **The open-source `features.yaml` wave is a demand signal.** The projects are all tiny (0-star repos) and appeared between 2025 and 2026, mostly framed as "memory for AI coding agents". Developers are inventing the feature registry because agents need feature-to-code context. No vendor has standardised a format, so a format or spec, an "OpenFeature for product features", is an open opportunity.
- **Reflag and Atono are the most dangerous fast-followers.** Reflag already treats "feature" as its noun, with stages and adoption. Atono already spans plan → flag → build and talks about rethinking user stories. Either could extend into specs and errors. Linear is the biggest incumbent threat because of its distribution plus Diffs, Releases and agents.
- **Azure DevOps comes closest structurally among incumbents** (Feature → dev links → deployment stages → tests), but it is a legacy planning hierarchy without production telemetry or code identity. That suggests the concept is not new; it lacked a code-native and telemetry-native implementation.

### Gaps
- Could not query the YC directory or Product Hunt search directly (hosts blocked or search not indexed by phrase). Some 2024–2026 YC companies in this niche may have been missed.
- Atono: no funding, founding team or traction data found. Its homepage and blog could not be fetched. Whether it links Sentry or analytics data per story is unverified.
- Tggl's "feature lifecycle management" page content could not be fetched.
- Reflag: round date and ARR or customer counts not found. Whether it links features to PRs, issues or specs is unverified.
- ProductMap AI: current status, pricing and funding unknown since its May 2025 launch.
- Augment Cosmos (May 2026) details came from a single Augment-authored page. Not verified whether it persists a feature-level record beyond specs.
- Factory's $150M Series C (Apr 2026) comes from a third-party comparison site only.
- No information found on Kitemaker's status, or on Cosine, Tusk or Sweep persisting feature-level records. These were searched but returned no relevant results.
- GitHub star counts and activity dates for the open-source feature-map repos could not be retrieved via the API (access blocked). Star counts come from page fetches where available.
