# Product-management and work-tracking platforms as competitors to an end-to-end per-feature lifecycle dashboard (state as of Nov 2026)

Method note: Research done 2026-11-19 through web search only. Direct fetches of linear.app and workmanagementhub.com were blocked by the session network, so vendor pricing and changelog pages could **not** be read directly. Most pricing below comes from third-party aggregators, which often disagree. Treat every price as "reported, unverified" unless marked otherwise. Anything dated before 2025 is flagged as possibly outdated.

## Q1. Does any of them show, per feature, the design, spec, code, release status and production health in one view? Which come closest?

### Takeaway
None of the tools surveyed shows design → spec → code → release → production health for a single feature in one native view. Linear comes closest on the engineering side: issue/project → PR (GitHub integration) → in-app code review (Diffs, May 2026) → deploy tracking (Releases / release pipelines, 2026). It still has no native production telemetry (errors, usage) or feature-flag panel. Jira comes closest on breadth, through its Development panel, Deployments, the feature-flag module, JPD and the Teamwork Graph, but it gets there by assembling integrations and marketplace apps, not through one opinionated per-feature view. Productboard and Aha! cover idea → spec → roadmap and push work into Jira or ADO. Productboard Pulse/Spark adds some usage analytics, but none of them follows code or deploys.

### Cited Findings

**Competitor matrix (lifecycle coverage)**

| Vendor | Idea/discovery | Spec/docs | Code (PR/commit link) | Release/deploy | Flags | Prod usage/errors | Notes |
|---|---|---|---|---|---|---|---|
| Linear | Partial (Initiatives/Projects, Asks) | Project docs | Yes, native GitHub PR linking + Diffs code review | Yes, Releases/release pipelines (2026) | No native panel found | No native panel found | Closest on eng side |
| Jira + JPD + Rovo | Yes (JPD) | Confluence | Yes (Development panel, via GitHub/GitLab/Bitbucket) | Yes (deployments module) | Yes (feature-flag module + LaunchDarkly/DevCycle/ConfigCat apps) | Via Teamwork Graph / Pendo (EA) | Broadest, but fragmented |
| Productboard | Yes (feedback, insights) | Yes | No (pushes to Jira/ADO) | No | No | Pulse (usage analytics, details unclear) | PM-side only |
| Aha! | Yes (Ideas) | Yes (Roadmaps, Knowledge) | Aha! Develop (eng tracker) | Release planning (not deploys) | No | No | PM-side, enterprise |
| Shortcut | Light | Docs | Yes (GitHub/GitLab) | No dedicated deploy tracking found | No | No | Korey AI agent |
| ClickUp | Light | Docs | GitHub integration | No | No | No | General work mgmt |
| Notion | Light | Strong (docs/specs) | GitHub links/sync | No | No | No | Spec home, not tracker |
| monday dev | Light | Docs | GitHub integration | No | No | No | General work mgmt |
| GitHub Projects/Issues | Light | Issues/markdown | Native | Native releases/deployments (GitHub) | No | No | Code-native, PM-weak |
| Asana | Light | Light | Integrations | No | No | No | Cross-functional work mgmt |
| Height | n/a | n/a | n/a | n/a | n/a | n/a | **Shut down Sept 24, 2025** |

Source basis for the matrix: the rows synthesize the findings cited below. "No" means no evidence was found in this session, not confirmed absence.

**Linear**
- Diffs (in-app PR review) launched May 28, 2026 on all plans. Linear framed it as a response to agent-generated PRs straining review. Features include structural diffing and AI-guided reviews, and it later came to mobile (approve, reply, send a fix back to an agent). — [Linear: Code review should be fast](https://linear.app/now/code-review-should-be-fast); [Sistava Linear AI review](https://sistava.com/en/ai-agent-platform-reviews/pm-ai/linear-ai)
- Linear Agent can run coding sessions ("Now Linear writes the code, too"). One third-party review says it writes code via Claude Code and Codex. — [Linear: coding sessions for Linear Agent](https://linear.app/now/coding-sessions-for-linear-agent); [Sistava](https://sistava.com/en/ai-agent-platform-reviews/pm-ai/linear-ai)
- Linear Releases: connects to CI/CD to track deployment environments, versions and issue statuses. It generates release notes and supports continuous and scheduled pipelines. Reported limits: up to 15 pipelines on Business, unlimited on Enterprise. — [megachangelog (Linear)](https://megachangelog.com/companies/linear)
- Each release pipeline has a Changelog tab for written or AI-generated notes. — [workmanagementhub (search snippet only; fetch blocked)](https://workmanagementhub.com/linear-releases-feature-ci-cd-deployment-tracking-2026/)
- CircleCI shipped a "Linear Releases" orb on Apr 30, 2026 that syncs deployments to Linear release pipelines, continuous and scheduled. — [CircleCI changelog](https://circleci.com/changelog/linear-releases-orb/)
- Changelog items: release-pipeline changelogs (Jul 2026); release search by version and release sync naming fixes (May 2026); pipeline team associations require pipeline admin rights (Sep 24, 2026). Launch date for Releases conflicts across trackers (Apr 30 vs Jun 11, 2026). — [Linear changelog via search](https://linear.app/changelog); [megachangelog](https://megachangelog.com/companies/linear)
- One tracker counts 128 Linear product updates in 2026. — [spyingbee](https://spyingbee.com/updates/linear/2026)
- A 2026 "Projects 2.0" reportedly added multi-team rollups, Gantt-style timelines and richer roadmap views (single third-party source). — [automationatlas Linear review 2026](https://automationatlas.io/answers/linear-review-2026/)
- Linear's CEO reportedly declared "issue tracking dead" as Linear moves toward agentic AI. — [pulse.bot](https://www.pulse.bot/saas/news/linear-moves-sideways-to-agentic-ai-as-ceo-declares-issue-tracking-dead-0e7e9bbd-23f1-4fe3-b7cb-9d1910ee7066)
- Linear links issues to GitHub PRs so statuses update automatically. Reviewers call its analytics "thin" for velocity, burndown and custom dashboards. — [aitoolpick Linear vs Jira 2026](https://aitoolpick.org/blog/linear-vs-jira-2026/); [wearefounders](https://www.wearefounders.uk/linear-vs-jira-2025-is-linear-finally-ready-for-non-technical-teams/)

**Atlassian (Jira, Jira Product Discovery, Rovo, Teamwork Graph)**
- At Team '26 (May 6, 2026) Atlassian opened the Teamwork Graph, which it says has more than 150B connections, to third-party tools via a Teamwork Graph CLI and Rovo MCP server tools (both open beta). Atlassian claims 44% more accurate results with 48% fewer tokens when answers are grounded in the graph. — [SiliconANGLE](https://siliconangle.com/2026/05/06/atlassian-opens-teamwork-graph-pushes-rovo-agentic-execution-team-26/); [ITBrief](https://itbrief.news/story/atlassian-opens-teamwork-graph-to-third-party-ai-tools)
- Agents in Jira became GA: agents can be assigned work items, with audit logging. — [SiliconANGLE](https://siliconangle.com/2026/05/06/atlassian-opens-teamwork-graph-pushes-rovo-agentic-execution-team-26/); [Capable](https://www.gocapable.com/blog/atlassian-team-2026-what-mattered-most)
- JPD Enterprise is GA, adding consistent intake, scorecards and roadmap rollups across many product teams. A new Feedback app is in early access. A "Product Collection" bundles JPD + Feedback + Rovo as an "AI-powered product operating system", and a Pendo integration is in early access (this is the production-usage link). — [Capable](https://www.gocapable.com/blog/atlassian-team-2026-what-mattered-most); [Buzzclan](https://buzzclan.com/atlassian/atlassian-collections-explained/)
- Rovo Studio is reported GA (no-code AI workflows/apps). — [CMO Tech](https://cmotech.ca/story/atlassian-widens-ai-push-with-teamwork-graph-rovo)
- Jira exposes a Feature Flag module so flag providers can attach flags to issues. DevCycle's Jira app shows per-environment flag status inside Jira (many-to-many tickets↔flags), and ConfigCat's app can toggle flags from an issue. — [Atlassian dev docs: Feature Flag module](https://developer.atlassian.com/cloud/jira/software/modules/feature-flag/); [DevCycle for Jira](https://marketplace.atlassian.com/apps/1227643/devcycle-feature-flag-management-for-jira); [ConfigCat for Jira](https://marketplace.atlassian.com/apps/1222421/configcat-feature-flags)

**Productboard**
- Productboard folded Productboard AI and Pulse into "Spark", included on every plan and metered by credits. Another source instead says Pulse is a usage-based analytics product sold through sales, priced on data volume. — [featq](https://featq.com/blog/productboard-pricing); [saastruecost](https://saastruecost.com/software/productboard-insights/pricing/)

**Height**
- Height announced its shutdown on Mar 20, 2025 (CEO Michael Villar) and stopped operating on Sept 24, 2025. No acquirer was found. Shortcut offered 6 months free to Height customers. — [AlternativeTo](https://alternativeto.net/news/2025/3/height-project-management-tool-to-shut-down-by-september-2025); [Shortcut import page](https://www.shortcut.com/import/height); [Rework](https://resources.rework.com/tools/project-management/best-height-alternatives)

**GitHub**
- Projects hierarchy view (sub-issues) reached GA on Mar 19, 2026. @copilot can be set as an assignee in issue templates. — [GitHub changelog](https://github.blog/changelog/2026-03-19-hierarchy-view-in-github-projects-is-now-generally-available/)
- Copilot "coding agent" was renamed "cloud agent" (Apr 2026). Assigning an issue produces a branch, code, tests and a PR. — [nxcode](https://nxcode.io/resources/news/github-copilot-complete-guide-2026-features-pricing-agents); [dev.to](https://dev.to/mr_manushukla/copilot-cloud-agent-in-2026-reasoning-levels-comment-triggers-and-the-ai-credit-math-behind-them-3jil)

### Inferences
- **Closest overall:** Jira plus marketplace apps (flags, deployments, Pendo EA, Confluence specs, JPD ideas), with the Teamwork Graph as the data layer. A per-feature view would still need custom dashboards or a Rovo agent. Atlassian is clearly heading toward a "graph of everything" that a per-feature dashboard could sit on, which makes it both the biggest threat and a possible integration target.
- **Closest engineering-native:** Linear, which now owns issue → PR review (Diffs) → agent coding → deploy (Releases). The missing pieces are flags, error rates, usage analytics and design files. That is exactly the "after release" half of the proposed SaaS. Linear's pace (128 updates in 2026) makes it the fastest-moving potential encroacher.
- PM-side tools (Productboard, Aha!) own the "why" (feedback, specs, roadmaps) but hand delivery off to Jira or ADO and lose sight of it after the handoff. Productboard's Pulse/Spark is the only PM-side move into production usage data found.
- General work-management tools (ClickUp, monday dev, Asana, Notion) only link to code at the PR-integration level. None showed deploy, flag or production-health linkage.
- Design (Figma) linkage exists in most tools as embeds or attachments. This session did not verify per-tool design-status integration.

### Gaps
- Could not read Linear's own docs to confirm whether Releases surfaces anything about production health (errors, metrics) or flag state. The evidence suggests it does not.
- No evidence found of a native Sentry/Datadog/PostHog health panel per issue or project in any tool. Integrations probably exist as link previews but were not verified.
- No verification of Aha! Develop's or Shortcut's deploy-tracking capabilities in 2025–2026.
- Linear "Insights" 2025–2026 changes and "Initiatives" updates were not found in the search results. One old note says Insights was added in 2023 (possibly outdated). — [Sistava](https://sistava.com/en/ai-agent-platform-reviews/pm-ai/linear-ai)

## Q2. How do they link work items to code (PR integrations, commits) and to production data (analytics/flags/errors)?

### Takeaway
Code linkage is commodity: nearly every tool auto-links PRs, branches and commits via an issue key in the branch or PR title and moves status on merge. Release linkage is rare: only Linear (Releases + CI) and Jira (deployments module) do it natively, and GitHub does it for repo releases. Production-data linkage is almost absent. Jira's feature-flag module plus third-party flag apps is the only structured mechanism found. Analytics comes via Productboard Pulse and a Jira–Pendo early-access integration. No tool found links error monitoring to a feature as a first-class field.

### Cited Findings
- Linear auto-links issues to GitHub PRs and syncs status. — [aitoolpick](https://aitoolpick.org/blog/linear-vs-jira-2026/)
- Linear Releases connects to CI/CD to track environments and versions per issue, and CircleCI has an official orb for it. — [megachangelog](https://megachangelog.com/companies/linear); [CircleCI](https://circleci.com/changelog/linear-releases-orb/)
- Linear Diffs puts code review inside the tracker, so the PR content (not just a link) lives next to the issue. — [Linear Now](https://linear.app/now/code-review-should-be-fast)
- Jira feature-flag module: flag providers attach flag state per environment to issues (DevCycle, ConfigCat apps). — [Atlassian dev docs](https://developer.atlassian.com/cloud/jira/software/modules/feature-flag/); [DevCycle](https://marketplace.atlassian.com/apps/1227643/devcycle-feature-flag-management-for-jira)
- Unleash recommends linking flags to tickets, code and dashboards (e.g., a flag ↔ Jira story ↔ Mixpanel/Grafana dashboard). This is a do-it-yourself pattern, not a packaged product. — [Unleash guide](https://www.getunleash.io/blog/feature-flag-driven-development-a-guide)
- Atlassian has a Pendo integration in early access within the Product Collection. — [Capable](https://www.gocapable.com/blog/atlassian-team-2026-what-mattered-most)
- Productboard Pulse is described as usage-based product analytics integration (custom pricing by data volume). — [saastruecost](https://saastruecost.com/software/productboard-insights/pricing/)
- monday dev's Standard plan includes GitHub integration, and integration actions draw from account-wide pools. — [Capterra monday dev pricing](https://www.capterra.com/p/10008569/monday-dev/pricing/); [xpay](https://www.xpay.sh/saas-pricing/monday-dev-platform/)
- GitHub: issues → Copilot cloud agent → PR, all native. Projects hierarchy is now GA. — [GitHub changelog](https://github.blog/changelog/2026-03-19-hierarchy-view-in-github-projects-is-now-generally-available/)

### Inferences
- The standard linkage key everywhere is the issue ID in the branch or PR. Release linkage then comes from walking commits between deploys (Linear Releases and Jira deployments both rely on CI telemetry). A per-feature dashboard can reuse this convention.
- The flag key ↔ feature mapping is the most natural bridge to production data. Atlassian standardized it with the flag module, and nobody else has. Errors and usage would need either a flag key, a route or component mapping, or an event-naming convention, and no incumbent provides that.
- The agent era (Linear Agent, Copilot cloud agent, Jira agents) multiplies PRs per feature. That raises the value of feature-level rollups over issue-level tracking.

### Gaps
- Not verified whether Linear has native Sentry, LaunchDarkly or PostHog integrations that show health inside an issue. They are not mentioned in the 2026 sources found.
- No per-tool data on Figma/design-status syncing.

## Q3. Pricing per seat per month for each tier (late 2026)

### Takeaway
Engineering trackers cluster at about $8–$16/seat/mo (Linear, Shortcut, Jira, monday dev, ClickUp). PM/roadmap tools cost far more: about $19–$59/maker (Productboard), $59+/user (Aha! Roadmaps). AI is moving to credit or usage metering (ClickUp Brain, Notion agents, Asana AI Studio, Productboard Spark, GitHub AI Credits). All figures are third-party unless noted, and many conflict.

### Cited Findings
- **Linear:** Free $0 (250 issues, 2 teams); Basic $10/user/mo annual (~$12 monthly); Business $16/user/mo annual (~$18 monthly); Enterprise custom. Reported 2026 increase from $8/$14. Some sources still show $8/$12, so this is unverified (linear.app fetch blocked). Releases: Business up to 15 pipelines, Enterprise unlimited. — [automationatlas](https://automationatlas.io/answers/linear-pricing-2026/); [activepieces](https://www.activepieces.com/blog/linear-app-pricing-every-plan-and-seat-cost-2026); [megachangelog](https://megachangelog.com/companies/linear)
- **Jira Cloud:** Free; Standard about $7.91 (300-user calculator, annual) to $8.48 (monthly, official page checked Nov 1, 2026) to $9.70 (1–100 users, monthly); Premium about $14.54 to $15.56 to $19.60; Enterprise custom. The variation comes from seat-count tiers and billing terms. — [automationatlas](https://automationatlas.io/answers/jira-pricing-explained-2026/); [costbench](https://costbench.com/software/project-management/jira/); [saaspricecheck](https://saaspricecheck.com/jira/pricing/)
- **Jira Product Discovery:** Free up to 3 creators; Standard $10/creator/mo; Premium $25/creator/mo; Enterprise custom (201+). Contributors are free, and annual billing saves up to about 17%. — [userjot](https://userjot.com/blog/jira-product-discovery-pricing); [featurebase](https://www.featurebase.app/blog/jira-product-discovery-pricing)
- **Productboard:** Conflicting reports. (a) Plus $19/maker annual ($25 monthly) and Business $59/maker annual ($75 monthly, 2-maker minimum), with 250/500 AI credits per maker. (b) As of Oct 2026 the pricing page shows only Business and Enterprise, with no prices. (c) A standalone Spark plan at $15/maker annual ($19 monthly) appeared briefly in spring 2026. Extra credits cost $5 per 50. — [featq](https://featq.com/blog/productboard-pricing); [theodorehq](https://www.theodorehq.com/docket/compare/productboard-review); [coldiq](https://coldiq.com/tools/productboard)
- **Aha!:** Roadmaps $59/user/mo (annual); Develop $9/user/mo; Enterprise reportedly $149/user/mo (unverified); Ideas often bundled. — [Vendr](https://www.vendr.com/marketplace/aha-faa15); [Capterra](https://www.capterra.com/p/144020/Aha/); [productlift](https://www.productlift.dev/blog/aha-pricing/)
- **Shortcut:** Free (up to 10 users); Team $8.50/user/mo annual ($10 monthly); Business $12 annual ($16 monthly); Enterprise custom. These figures cite Shortcut's page as of Jul 27, 2026. Korey AI agent is priced per org: Starter $59/mo (250 interactions), Growth $149/mo (1,000), Enterprise custom. — [Taskfolk](https://taskfolk.ai/blog/shortcut-alternatives); [G2 Korey pricing](https://www.g2.com/products/korey/pricing)
- **ClickUp:** In transition. New workspaces (Oct 2026): Core $9, Business $17/user/mo annual ($12/$25 monthly), with AI credits included, plus a new Max plan at $100. Classic plans: Unlimited $7, Business $12 annual ($10/$19 monthly). Brain AI add-on $9/user, Everything AI $28/user, extra credits $10 per 10k. — [usecarly](https://www.usecarly.com/blog/clickup-pricing/); [get-alfred](https://get-alfred.ai/blog/clickup-pricing); [eesel](https://www.eesel.ai/blog/clickup-pricing)
- **Notion:** Business about $20/member/mo annual ($24 monthly), with Notion AI bundled. The standalone $10 AI add-on was retired in May 2025. Custom Agents use credits at about $10 per 1,000 (one source says $13 annual). Plus and Free tiers not detailed here. Enterprise custom. — [get-alfred Notion](https://get-alfred.ai/blog/notion-pricing); [toolhunter](https://toolhunter.ai/blog/notion-ai-pricing)
- **Asana:** Starter $10.99/user/mo annual ($13.49 monthly); Advanced $24.99 ($30.49); Enterprise custom. AI Studio is bundled with account-level credits (50k/75k/200k). AI Studio Plus costs $135/account/mo annual. — [atrix](https://atrix.dev/en/blog/asana-pricing-2026-explained); [spendhound](https://www.spendhound.com/marketplace/asana-pricing)
- **monday dev:** Reports conflict. Basic about $9; Standard about $12–$14/seat/mo annual; Pro about $20–$23; 3-seat minimum; monthly billing about 18% higher; Enterprise custom. — [Capterra](https://www.capterra.com/p/10008569/monday-dev/pricing/); [GetApp](https://www.getapp.com/project-management-planning-software/a/monday-dev/)
- **GitHub:** Projects/Issues are bundled with GitHub plans (seat prices not re-verified this session). Copilot moved to usage-based "GitHub AI Credits" ($0.01 each) on Jun 1, 2026, and the cloud agent consumes credits. Copilot tiers reported at $10 (Pro) to $39 (Pro+), with Business $19 and paid plans up to about $100/user. Agent availability per tier conflicts across sources. — [gapvelocity](https://www.gapvelocity.ai/blog/github-copilots-new-usage-based-billing-what-changed-why-developers-are-upset-and-what-it-means); [userjot Copilot](https://userjot.com/blog/github-copilot-pricing-guide-2025)

### Inferences
- A per-feature dashboard sold to the PM/eng-leadership buyer could price above trackers ($8–$16) and near or below PM tools ($19–$59/maker). Charging per "maker/viewer" or per feature tracked, rather than per engineer seat, would avoid competing with tracker seat budgets.
- Every incumbent is adding AI credit metering, so buyers are being conditioned to accept usage-based AI charges.

### Gaps
- No vendor pricing page was read directly. Linear, Productboard, monday dev and GitHub seat prices are especially uncertain.
- Notion Free/Plus and GitHub Team/Enterprise seat prices were not captured.

## Q4. What do users complain about regarding feature-level visibility after shipping?

### Takeaway
Searches did not surface primary Reddit or HN threads specifically about "no visibility after a feature ships". The available evidence is secondhand. Linear is criticized for thin analytics and basic roadmaps, Jira for complexity and cost, and PM tools for per-seat cost. The "post-ship blind spot" is better described as structural: no tool here tracks adoption or health per feature, so teams stitch it together with flags and analytics dashboards. This is a weakly evidenced finding; the report should not overstate user-voiced demand.

### Cited Findings
- Linear: "if you need detailed velocity charts, burndown reports, or custom dashboards, Linear's analytics will feel thin." — [wearefounders](https://www.wearefounders.uk/linear-vs-jira-2025-is-linear-finally-ready-for-non-technical-teams/)
- Linear's roadmap "will feel like a toy" to a VP expecting Gantt charts, dependencies and critical path. Cross-team dependencies and quarterly resource allocation are outside its scope. — [dev.to Linear review](https://dev.to/pickuma/linear-review-project-management-built-for-software-teams-48en); [ClickUp blog (vendor content)](https://clickup.com/blog/linear-vs-jira/)
- A summary of HN discussion says Jira remains the default when advanced roadmapping, permissions or compliance matter (vendor blog, secondhand). — [ONES blog](https://ones.com/blog/?p=1059027)
- Aha!: reviewers say per-user pricing makes broad access expensive. — [productlift](https://www.productlift.dev/blog/aha-pricing/); [Capterra Aha! reviews](https://www.capterra.com/p/144020/Aha/)
- Productboard: the credit-based AI system could produce unexpected costs for heavy users. — [toolradar](https://toolradar.com/tools/productboard)
- GitHub Copilot's move to usage billing upset developers. — [gapvelocity](https://www.gapvelocity.ai/blog/github-copilots-new-usage-based-billing-what-changed-why-developers-are-upset-and-what-it-means)
- Product literature frames "features shipped" becoming the success metric as the "build trap", which is the gap post-release visibility addresses. — [Escaping the Build Trap notes (Goodreads)](https://www.goodreads.com/notes/42611483-escaping-the-build-trap/32430294-cleo-morgause/3ac21315-68b1-482a-b3cf-0762bd8207dc)

### Inferences
- The pain is more likely felt by PMs and eng leaders, as "did it ship, is it on, is anyone using it, is it breaking?", than voiced as a tool complaint. It shows up as tool sprawl: the tracker plus LaunchDarkly, Amplitude or PostHog, plus Sentry or Datadog.

### Gaps
- No direct G2, Reddit or HN quotes about post-ship visibility were retrieved, because search returned no matching threads. This needs a targeted manual pass (e.g., r/ProductManagement "feature adoption tracking", HN "Linear releases").

## Q5. What did vendors launch in 2025–2026 that moves toward the end-to-end vision? Plus traction, funding and buyers

### Takeaway
2025–2026 launches converge on agents doing the delivery work: Linear Agent/Diffs/Releases, Jira agents, Copilot cloud agent, Shortcut Korey, ClickUp Super Agents, Notion Custom Agents and Asana AI Studio. Two vendors also build a data graph across tools (Atlassian Teamwork Graph, opened via MCP and CLI). Linear and Atlassian are the two moving toward a full idea → release chain. Productboard is shrinking (30% layoff in Apr 2026) while pivoting to AI. Height is dead. No well-funded 2025–2026 startup explicitly pitching a "feature lifecycle dashboard" was found.

### Cited Findings
- **Linear:** Diffs (May 28, 2026); Linear Agent coding sessions; Releases/release pipelines with AI changelogs (2026); Projects 2.0 multi-team rollups (reported). Funding: $82M Series C, Jun 2025, led by Accel, at $1.25B valuation, about $134M total raised. A reported Aug 2026 doubling to $2.5B is unconfirmed (Business Insider via secondary sources). Buyer: software and product teams at startups through mid-market, moving upmarket. — [Linear Now](https://linear.app/now/code-review-should-be-fast); [stockanalysis](https://stockanalysis.com/private/linear/); [Sacra](https://sacra.com/c/linear/)
- **Atlassian:** Team '26 (May 6, 2026) launched Teamwork Graph opened to outside agents (CLI + MCP, open beta), Agents in Jira GA, Rovo Studio GA, JPD Enterprise GA, Feedback app (EA), Product Collection, and Pendo integration (EA). Buyer: enterprise IT, engineering and product orgs. — [SiliconANGLE](https://siliconangle.com/2026/05/06/atlassian-opens-teamwork-graph-pushes-rovo-agentic-execution-team-26/); [Capable](https://www.gocapable.com/blog/atlassian-team-2026-what-mattered-most); [theaieconomy](https://theaieconomy.substack.com/p/atlassian-teamwork-graph-open-rovo-ai-agents)
- **Productboard:** Spark AI agent (public beta, 2026) absorbed Productboard AI and Pulse. Laid off about 30% of staff on Apr 15, 2026 as it bet on AI. Total funding about $261–262M, including a $125M Series D at $1.725B (2022, possibly outdated). Buyer: product-management orgs, mid-market and enterprise. — [lupa.cz](https://www.lupa.cz/aktuality/productboard-propousti-tretinu-zamestnancu-radeji-rez-nez-pomale-krvaceni-rika-palan-o-ai-modelu-firmy/); [cc.cz](https://cc.cz/dramaticky-rez-v-prednim-ceskem-startupu-propousti-pres-30-tymu-a-sazi-vse-na-umelou-inteligenci/); [CB Insights](https://www.cbinsights.com/research/productboard-series-d-funding); [featq](https://featq.com/blog/productboard-pricing)
- **Aha!:** Bootstrapped. Last official figures: 5,000+ customers and a $40M run rate (2018), with ARR above $100M per the CEO in late 2021 (both possibly outdated). Buyer: enterprise product management. — [Aha! blog](https://www.aha.io/blog/40m-revenue-5000-customers); [yespress](https://yespress.io/aha-made-bootstrapping-product-feature)
- **Shortcut:** Korey AI agent for product development (per-org pricing). About $39M raised, with the last round a $25M Series B led by Greylock in Jan 2020 (possibly outdated, no later rounds found). Buyer: small and mid-size software teams. — [rywalker Korey](https://rywalker.com/research/korey); [Shortcut Series B](https://shortcut.com/blog/building-the-next-stage-of-shortcut-25m-in-series-b-funding); [CB Insights](https://www.cbinsights.com/deal/clubhouse-software-seed-vc)
- **ClickUp:** Brain AI, "Super Agents", and a new plan structure (Core/Business/Max) in Oct 2026. — [workmanagementhub (snippet)](https://workmanagementhub.com/clickup-brain-ai-complete-guide-2026/); [usecarly](https://www.usecarly.com/blog/clickup-pricing/)
- **Notion:** AI bundled into Business (May 2025), with Custom Agents metered by credits. — [toolhunter](https://toolhunter.ai/blog/notion-ai-pricing)
- **Asana:** AI Studio bundled with credits, plus AI Studio Plus/Pro add-ons. — [gravity.fast](https://gravity.fast/blog/asana-ai-agent/)
- **GitHub:** Projects hierarchy GA (Mar 2026); Copilot cloud agent (renamed Apr 2026); usage billing (Jun 2026). — [GitHub changelog](https://github.blog/changelog/2026-03-19-hierarchy-view-in-github-projects-is-now-generally-available/); [gapvelocity](https://www.gapvelocity.ai/blog/github-copilots-new-usage-based-billing-what-changed-why-developers-are-upset-and-what-it-means)
- **Height:** shut down Sept 24, 2025 after pivoting to "AI-autonomous" project management. — [AlternativeTo](https://alternativeto.net/news/2025/3/height-project-management-tool-to-shut-down-by-september-2025)
- **Startups:** The search found no startup branded around "feature lifecycle" or "product delivery visibility". The nearest hit is AI Library, which raised a $560K pre-seed (May 2026) at a $7.5M cap to automate the enterprise software-delivery lifecycle with AI agents. Adjacent but different: Qodo (AI code review, $70M Series B, Mar 2026). — [The AI Insider](https://theaiinsider.tech/2026/05/09/ai-library-raises-pre-seed-funding-to-automate-enterprise-software-delivery-with-ai-agents/); [Qodo (Wikipedia)](https://en.wikipedia.org/wiki/Qodo)

### Inferences
- Linear is the most direct threat. It already covers project → PR → review → deploy and has the momentum and capital (possibly $2.5B valuation) to add flag and error panels. A new entrant's moat would have to be the post-release half (flags + usage + errors per feature) and cross-tool neutrality: working across Jira, Linear and GitHub at once.
- Atlassian's open Teamwork Graph and MCP could make a per-feature dashboard easier to build on, as an integration surface, and also easier for Atlassian to ship itself.
- Productboard's contraction and Height's death suggest PM-tool consolidation. Standalone PM-side tools without a code or production link are under pressure.
- Adjacent categories not covered here (engineering intelligence such as LinearB, Swarmia, Jellyfish, DX; developer portals such as Port and Cortex; flag platforms such as LaunchDarkly and Statsig, which own release + metrics) are likely closer competitors on the "release → monitoring" half and should be covered by other researchers.

### Gaps
- No 2025–2026 startup explicitly pitching an end-to-end per-feature dashboard was found. It is unclear whether this is real absence or search limits; a targeted Product Hunt or YC W25–F26 batch search is recommended.
- Revenue and traction for Linear (ARR), ClickUp, monday dev and Notion for 2026 were not captured.
- Linear's $2.5B valuation (Aug 2026) is unconfirmed.
