# Competitors: design → spec → code traceability (ALM, design handoff, AI spec-driven tools)

Research date: 2026-11-19. Scope: classic ALM/requirements traceability, Figma-centred design-to-code linkage, and the 2025–2026 AI spec-driven development (SDD) wave, assessed as competitors to a per-feature dashboard covering design → spec → implementation → release → monitoring.

Method note: 17 web searches, no full-page fetches (codemyspec.com, a useful SDD comparison site, was not reachable from this session's network). Most pricing for ALM tools comes from aggregators (G2, Capterra, ITQlick, SourceForge), not vendor price sheets. Several 2026 SDD sources are vendor or affiliate blogs. These limits are flagged next to the relevant findings.

## Q1. Do ALM tools reach into production monitoring? Why are they mostly confined to regulated industries, and what do they cost?

### Takeaway
Classic ALM/RM tools (Jama, Polarion, Codebeamer, DOORS Next, Helix ALM) trace requirements → risk → tests → release for compliance (ISO 26262, IEC 62304). The closest any of them gets to "production" is a release or deployment record. None of the sources found show them linking to runtime monitoring or observability. They sell on audit evidence in regulated industries, prices are quote-based, and the published estimates run from roughly $40 to $165 per user per month, with enterprise contracts near $75k/yr. Azure DevOps comes closest to the dev-to-deploy chain (work item ↔ commit ↔ PR ↔ build ↔ release ↔ environment deployment), but it also stops at deployment.

### Cited Findings
**Lifecycle coverage**
- In regulated sectors, compliance with ISO 26262 and IEC 62304 "requires live traceability across requirements, risk, test, and release". The analysis puts Siemens, IBM and PTC in the enterprise ALM segment and Jama Connect among requirements-centric platforms for regulated industries. — [QKS Group review: the split in ALM for regulated industries](https://qksgroup.com/blogs/qks-review-the-split-in-alm-for-regulated-industries-enterprise-giants-vs-focused-specialists-1484)
- Jama keeps traceability between requirements and tests but is "more focused on the requirements side of the equation". Codebeamer "automatically link[s] requirements to test cases". — [Inflectra: Codebeamer vs Jama Connect](https://www.inflectra.com/tools/comparisons/codebeamer-vs-jama-connect)
- Users praise Jama's "live traceability". G2 reviewers call Polarion's traceability features "particularly impressive". — [G2: Jama vs Codebeamer](https://www.g2.com/compare/jama-connect-for-requirements-management-vs-codebeamer); [G2: Polarion vs Codebeamer](https://www.g2.com/compare/polarion-vs-codebeamer)
- Azure DevOps "supports end-to-end traceability by linking various objects in your development process": work items link to branches, commits, pull requests, builds and releases. — [Microsoft Learn: End-to-end traceability](https://learn.microsoft.com/en-in/azure/devops/cross-service/end-to-end-traceability?view=azure-devops)
- YAML deployment jobs that target an environment fill in the Deployment control on linked Azure Boards work items with live environment status. This comes from a community AZ-400 study guide, not Microsoft docs. — [open-exam-prep AZ-400 guide](https://open-exam-prep.com/study-guides/azure-az-400/flow-of-work-traceability/end-to-end-traceability)
- Modern Requirements4DevOps adds requirements governance inside Azure DevOps and builds "live matrices directly from your Azure DevOps work items". Its vendor blog positions Azure DevOps as the control layer for "planning, requirements, traceability, test orchestration, governance, and release discipline" even when the code lives in GitHub. — [Modern Requirements: traceability](https://www.modernrequirements.com/products/modern-requirements4devops-features/traceability/); [Modern Requirements blog on GitHub/Azure DevOps](https://www.modernrequirements.com/blogs/microsoft-github-and-azure-devops-what-the-shift-really-means/)
- GitLab and Azure DevOps have no native connection. Third parties (Canarys GLAB Integrator, Getint) sync GitLab activity to Azure Boards over webhooks. Inflectra SpiraTeam claims "every code change can be linked back to its originating requirement or defect". — [Canarys](https://canarysdevopsolutions.ecanarys.com/GitLab–Azure_Boards_Integrator); [Getint](https://www.getint.io/integrations/azure-devops-gitlab); [Inflectra SpiraTeam–GitLab](https://inflectra.com/Products/SpiraTeam/Integrations/GitLab.aspx)

**Market and positioning**
- Jama claims to be "#1 in Market Share for Requirements Management". That is a vendor claim. — [Jama Software](https://www.jamasoftware.com/?p=57134)
- PeerSpot mindshare (based on user engagement, not revenue): Jama Connect 10.4% and PTC Integrity Requirements Connector 2.9% in application requirements management. — [PeerSpot](https://www.peerspot.com/products/comparisons/jama-connect_vs_ptc-integrity-requirements-connector)
- 2026 roundups name Polarion "best for Siemens PLM ecosystems" and Codebeamer "best for software-centric ALM" / regulated agile, and favour Jama for live traceability in medical devices. — [TNW: medical device RM 2026](https://thenextweb.com/news/best-requirements-management-tools-medical-device-2026); [TNW: automotive RM 2026](https://thenextweb.com/news/automotive-requirements-management-2026-7-solutions-compared)
- Frost & Sullivan brief (hosted by Jama): "Modern Requirements Management Solutions are Necessary for Highly Regulated Industries". — [Jama/Frost & Sullivan](https://www.jamasoftware.com/?p=9814)

**Pricing (no vendor publishes list prices; all figures are third-party and inconsistent)**
- Codebeamer and Jama Connect both use "custom pricing models that are not publicly disclosed". — [Inflectra](https://www.inflectra.com/tools/comparisons/codebeamer-vs-jama-connect)
- Codebeamer: $1,500 per user per year (RM or TM plan) and $1,800 per user per year (LM plan), billed annually. — [Capterra: codebeamer](https://www.capterra.com/p/92177/codeBeamer/)
- Polarion X Base: entry price from $42/month. — [G2: Polarion vs Codebeamer](https://www.g2.com/compare/polarion-vs-codebeamer)
- ITQlick estimates: Jama about $50, Polarion about $60 and Codebeamer about $79 per user per month. SourceForge lists $102 per user per month for all three, which may be a placeholder. A PeerSpot reviewer reported Jama at about €50–60 per user. — [ITQlick: codebeamer pricing](https://www.itqlick.com/codebeamer/pricing); [SourceForge comparison](https://sourceforge.net/software/compare/Jama-vs-Polarion-REQUIREMENTS-vs-codeBeamer/); [PeerSpot: Jama vs Polarion](https://www.peerspot.com/products/comparisons/jama-connect_vs_polarion-alm)
- Jama Connect: estimated $76,543/yr median contract (aggregator). IBM DOORS Next: G2 lists Professional SaaS from 164.00 per user per month, while another listing says pricing is "sales-led". Helix ALM: $89/user/mo on one aggregator and "per user quote-based" on another. — [checkthat.ai](https://checkthat.ai/answers/what-are-the-best-requirements-management-software-solutions); [G2: DOORS Next pricing](https://www.g2.com/products/ibm-engineering-requirements-management/pricing); [xpay.sh: DOORS Next](https://www.xpay.sh/saas-pricing/ibm-engineering-requirements-management-doors-next/); [ONES.com](https://ones.com/blog/?p=1146914)

### Inferences
- ALM tools justify their price through compliance evidence (traceability matrices and audit trails for ISO 26262, IEC 62304, DO-178C). Most commercial software teams face no such mandate, so the ROI for them is weak. That plus quote-based enterprise pricing and heavy process overhead explains why adoption is concentrated in regulated industries.
- Their traceability model is requirement ↔ test ↔ release. "Done" means verified and released, not "healthy in production", so post-release telemetry (errors, usage, SLOs per feature) is outside their data model.
- Azure DevOps shows the dev-to-deploy linking is technically easy inside one vendor's stack. It still has no design-artifact or spec-document layer and no runtime-health view per feature.
- Opening for the proposed SaaS: "ALM-grade traceability for unregulated, AI-heavy product teams", priced per seat like dev tools ($10–30/user) rather than $50–165/user, with release and monitoring links that ALM tools do not have.

### Gaps
- No vendor documentation was found showing any ALM tool (Jama, Polarion, Codebeamer, DOORS Next, Helix ALM) integrating with observability tools such as Datadog, Sentry or New Relic to show per-requirement production health. Absence in these searches is not proof that no such integration exists.
- GitLab's native requirements management (requirement objects, test-report satisfaction) and Value Stream Analytics were not covered by any source found. Background knowledge, unverified here: GitLab requirements are a Premium/Ultimate feature and VSA measures issue → MR → deploy cycle time, but not at a per-spec level.
- No official 2026 price sheets were found. Every ALM price above is an aggregator estimate.
- No audited market-share or revenue data by vendor.

## Q2. How close is Figma (Dev Mode + Code Connect + MCP) to linking a design to the implemented code and its status?

### Takeaway
Figma links design components to code components through Code Connect mappings and serves them to AI agents through the MCP server (GA since October 2025). At Config 2026 (June) it added two-way canvas ↔ repository sync: Code Layers in closed beta, plus Git branching and commits in Figma Make. Coverage is component-level and design-system-level, not feature-level. No source shows Figma tracking whether a particular screen or feature has been implemented, merged, released, or how it behaves in production.

### Cited Findings
- Code Connect UI and CLI both feed Figma's MCP server, which makes connected components available to AI-enabled tools and code editors. — [Figma Help: Code Connect](https://help.figma.com/hc/en-us/articles/23920389749655-Code-Connect)
- Mappings are shared with the MCP server immediately and used as context. — [Figma Developers: Code Connect UI setup](https://developers.figma.com/docs/code-connect/code-connect-ui-setup/)
- On Professional plans only one framework is active at a time. Organization and Enterprise plans can have several. Connections for a previous framework show a "Deactivated" badge and are not returned by the MCP server. — [Figma Help: Code Connect](https://help.figma.com/hc/en-us/articles/23920389749655-Code-Connect)
- The remote MCP server (mcp.figma.com) is recommended and available on all seats and plans. The desktop server needs a Dev or Full seat on a paid plan. — [Figma Developers: MCP tools and prompts](https://developers.figma.com/docs/figma-mcp-server/tools-and-prompts/); [mcpverdict](https://mcpverdict.com/mcp/servers/design/figma/)
- The MCP server launched in beta in June 2025 ("Dev Mode MCP server") and has been generally available since Schema 2025 (October 2025). — [AlternativeTo, June 2025](https://alternativeto.net/news/2025/6/figma-launches-dev-mode-mcp-server-for-direct-ai-access-to-design-data); [TFC](https://www.thefrontendcompany.com/posts/figma-mcp-server)
- Config 2026 (June 24, 2026) introduced Code Layers. Teams can work with code inside the Design canvas, including cloning repositories and syncing changes back to code. It is a closed beta; sources give the rollout start as July 2026 or as a waitlist. — [CMSWire](https://www.cmswire.com/digital-experience/figma-launches-code-layers-motion-at-config-2026/); [Figma Help: Config 2026 guide for admins](https://help.figma.com/hc/en-us/articles/41301052048791-Config-2026-a-guide-for-admins); [Startup Fortune](https://startupfortune.com/figma-turns-its-design-canvas-into-a-coding-environment-at-config-2026/)
- Figma Make gained Git support at Config 2026 (create branches, revert commits, manage version control) and a canvas-to-code loop that copies Make screens into Figma Design and syncs edits back to the codebase. — [makebttr: Config 2026](https://www.makebttr.com/insights/figma-config-2026); [rango.design](https://rango.design/articles/figma-config-2026-handoff)
- Dev Mode motion inspection is in open beta, with code copy to CSS, JSON or React. Animation context goes to coding agents through the MCP server. — [explainx Config 2026 recap](https://explainx.ai/blog/figma-config-2026-complete-recap-motion-code-shaders-ai-2026) (third-party recap; details vary across sources)
- Zeplin's Storybook integration "automatically highlight[s] and link[s] all instances of the designed component to the live implementation in Storybook". This is the closest any handoff tool comes to showing implementation linkage. — [Zeplin blog](https://blog.zeplin.io/inside-zeplin-powered-up-storybook-integration)
- Supernova added native Storybook support for design-system documentation. — [Supernova blog](https://www.supernova.io/blog/storybook-for-designers-why-its-more-than-just-a-dev-tool)
- Chromatic snapshots every component and flags pixel differences on each pull request. It is visual regression testing, not handoff status tracking. — [dupple: best design handoff tools 2026](https://dupple.com/learn/best-design-handoff-tools)

### Inferences
- Figma is moving fast from handoff toward shared design–code artifacts (Code Layers, Make with Git). That makes it a serious threat at the design ↔ code edge, especially if it adds Git/PR status to frames.
- Even so, Figma's unit is the component or file, not the feature or user story. It has no spec layer, no release or deploy state, and no production metrics, and nothing shows it building toward lifecycle status.
- Code Connect is valuable as an integration source for the proposed SaaS. Its mappings give a machine-readable design ↔ code join that a per-feature dashboard could ingest rather than rebuild.
- Zeplin and Storybook link designs to living component implementations, but only at design-system level. Chromatic gives per-PR UI change status, which could be one signal in a dashboard.

### Gaps
- No primary Figma blog post for Config 2026 was retrieved, so specific feature names and rollout dates come from third-party recaps and the Figma admin help article.
- Not covered by any source found: Anima, Builder.io (Visual Copilot / Fusion), Supernova pricing, Zeplin pricing and 2026 AI features (one review claims Zeplin added AI code generation, unverified), Figma seat pricing for Dev Mode.
- No evidence either way on whether Figma plans to show PR, deploy or issue status on frames. A dedicated look at Figma's Jira/GitHub/Linear integrations is needed.

## Q3. Which spec-driven tools persist specs per feature and track whether code still matches them? Funding and traction?

### Takeaway
Most SDD tools save specs as Markdown files in the repository, per feature or per change: Kiro `.kiro/specs/<feature>/`, Spec Kit per-feature spec/plan/tasks, OpenSpec living specs plus change deltas, BMAD agent-generated docs. Drift detection is weak. It is manual (Spec Kit's `/speckit.reconcile` extension, OpenSpec `/opsx:sync`, BMAD QA-agent review) or comes from small open-source CLIs. Augment Intent's "living specs" with a verifier agent is the main commercial claim of automatic spec ↔ code sync. No SDD tool found links specs to release or production. Tessl is the best-funded player ($125M) but still not GA by mid-2026. Kiro is the main commercial IDE, with 100k+ developers on its waitlist in its first 90 days.

### Cited Findings
**AWS Kiro**
- The spec workflow writes requirements.md (or bugfix.md), design.md and tasks.md into a per-feature folder under `.kiro/specs/`. — [Kiro pricing/specs overview (Rundown/Finout reviews)](https://www.finout.io/blog/what-is-kiro)
- Kiro reached GA on November 17, 2025, adding team sign-up through AWS IAM Identity Center and a CLI. Sources conflict: some pages give 7 May 2026 as GA, but AWS-adjacent reporting supports November 2025. — [SiliconANGLE, Nov 17 2025](https://siliconangle.com/2025/11/17/aws-launches-kiro-general-availability-team-features-cli-support/); [kirotutorial timeline](https://kirotutorial.com/history/)
- Traction: "since its launch more than 90 days ago, more than 100,000 developers have joined the waitlist" (AWS, October 20, 2025). A pricing-history page cites a Kiro one-year retrospective saying more than 100,000 developers tried the IDE in its first 5 days. — [AWS News Blog weekly roundup, Oct 20 2025](https://aws.amazon.com/blogs/aws/aws-weekly-roundup-kiro-waitlist-ebs-volume-clones-ec2-capacity-manager-and-more-october-20-2025); [usagepricing.com](https://www.usagepricing.com/blueprint/kiro)
- Pricing (verified against kiro.dev on July 4, 2026 per one review): Free $0 (50 credits/mo), Pro $20 (1,000), Pro+ $40 (2,000), Pro Max $100 (5,000), Power $200 (10,000), with overage at $0.04/credit. Enterprise is custom and billed through AWS. — [morphllm: Kiro pricing, Sept 2026](https://www.morphllm.com/kiro-pricing); [axonbuild review](https://axonbuild.com/blog/kiro-ide-review/)
- AWS has extended Kiro with "Kiro Crew" long-running bots and agent orchestration. — [Crypto Briefing](https://cryptobriefing.com/aws-kiro-crew-bots-orchestration/) (secondary source)
- No 2026 user, revenue or market-share figures for Kiro were found. — (see Gaps)

**Tessl**
- Raised $125M ($25M seed + $100M Series A led by Index Ventures, with Accel, GV and boldstart). Sources disagree on timing (announced November 2024 vs. 2025). Reported valuation was over $500M, or $750M per Fortune. — [pmf.show](https://www.pmf.show/blog/tessl-guy-podjarny-125m-spec-driven-development-ai-native); [AI CERTs](https://www.aicerts.ai/news/developer-tools-funding-tessl-raises-125m-for-autonomous-coding/); [Calcalist](https://www.calcalistech.com/ctechnews/article/i7ucn8teu)
- No new 2026 round found. A mid-2026 review says the Tessl Framework "still does not appear to have reached general availability". Another describes the main engine as closed-beta `tessl build` (lower-authority source). — [codemyspec Tessl review 2026](https://codemyspec.com/blog/tessl-review); [spec-compare on GitHub](https://github.com/cameronsjo/spec-compare/blob/main/docs/tools/tessl.md)

**GitHub Spec Kit, OpenSpec, BMAD (open source)**
- Spec Kit: drift handling is newer. A community drift-reconciliation extension (`/speckit.reconcile`) has to be triggered and reviewed by hand. — [dev.to: Spec Kit vs BMAD vs OpenSpec 2026](https://dev.to/willtorber/spec-kit-vs-bmad-vs-openspec-choosing-an-sdd-framework-in-2026-d3j); [developertoolkit.ai](https://developertoolkit.ai/en/ecosystem/frameworks/spec-driven-frameworks/)
- spec-kit-sync (community extension) reports requirements where spec and code disagree, features that exist in code without a spec, and contradictions between specs. It proposes whether to update the spec or fix the code. — [GitHub: bgervin/spec-kit-sync](https://github.com/bgervin/spec-kit-sync)
- OpenSpec keeps one living spec and changes it one delta at a time, archiving changes. `/opsx:sync` resyncs, but specs do not update themselves during implementation. — [specroot.io](https://specroot.io/notes/spec-kit-vs-openspec-vs-bmad/); [aicodingpatterns](https://aicodingpatterns.com/en/patterns/openspec-vs-spec-kit-vs-bmad/)
- BMAD has no built-in drift verification. Drift checks are done by hand through a QA agent review. — [tooltwist CxO guide](https://tooltwist.com/insights/spec-driven-frameworks-cxo-guide); [redreamality](https://redreamality.com/blog/-sddbmad-vs-spec-kit-vs-openspec-vs-promptx/)
- Benchmark: Spec Kit showed a 12.5% spec drift rate in Uvik's test runs (vendor blog benchmark, methodology unverified). — [Uvik SDD benchmark 2026](https://uvik.net/blog/spec-driven-development-benchmark/)
- Thoughtworks Radar Vol 35 (November 2026): spec-driven development stays in Assess (first listed November 2025). OpenSpec moved from Assess to Trial, praised for brownfield codebases and for retrofitting specs onto existing code. GitHub Spec Kit was in Assess (April 2026 blip). "AI-driven documentation sprawl", where teams use Spec Kit or BMAD to generate large volumes of Markdown, is placed in Caution. — [Thoughtworks Radar Vol 35 PDF](https://www.thoughtworks.com/content/dam/thoughtworks/documents/radar/2026/11/tr_technology_radar_vol_35_en.pdf); [Radar: OpenSpec](https://www.thoughtworks.com/radar/tools/openspec); [Radar: GitHub Spec Kit](https://www.thoughtworks.com/radar/languages-and-frameworks/github-spec-kit); [Radar techniques](https://www.thoughtworks.com/radar/techniques)

**Augment Code Intent**
- Public beta since February 2026 (macOS first). Multi-agent orchestration with "living specs that stay synchronized as agents work": a Coordinator delegates to parallel Implementer agents sharing a living specification, a Verifier checks completed work, and a reviewer agent can detect divergence between the living spec and the code. Bring-your-own agents (Claude Code, Codex) can connect, but the full Context Engine needs an Augment subscription. Sources are mostly Augment's own comparison pages. — [Augment: Intent vs Kiro](https://www.augmentcode.com/tools/intent-vs-kiro); [Augment: Intent vs GitHub Spec Kit](https://www.augmentcode.com/tools/intent-vs-github); [Product Hunt discussion](https://www.producthunt.com/p/augment-code/intent-by-augment-code-is-spec-driven-multi-agent-development-the-next-step-after-the-ide)

**Drift-detection niche (mostly open source)**
- SpecSync (ChrysKoum): a commit-time quality gate that keeps specs, code, tests and docs in sync and blocks drifting commits. — [GitHub: chryskoum/specsync](https://github.com/chryskoum/specsync)
- CorvidLabs spec-sync: a Rust CLI and MCP server for bidirectional drift detection between Markdown module specs and code that "fail[s] CI when code drifts". — [GitHub: CorvidLabs/spec-sync](https://github.com/CorvidLabs/spec-sync)
- oas-drift: a dependency-free CLI that finds OpenAPI ↔ route drift without an LLM. — [dev.to](https://dev.to/sunnydachs/your-openapi-spec-is-a-contract-nobody-is-checking-it-i-built-a-cli-that-detects-spec-code-drift-23ah)
- A Claude Code "Drift Detection" skill compares the implementation against PRDs and design docs and keeps a drift log. — [mcpmarket](https://mcpmarket.com/tools/skills/drift-detection)
- SpecStory and TrueSpec publish "spec drift" explainer content, which suggests products positioned on the term. — [SpecStory: What is spec drift?](https://specstory.com/learning/ai-coding/spec-drift); [TrueSpec: Spec drift 101](https://truespec-app.com/learn/spec-drift-101)

### Inferences
- "Specs per feature in the repo" is now standard practice (Kiro, Spec Kit, OpenSpec, Claude Code/Cursor plan files). A dashboard could index these files rather than ask teams to author specs in a new tool. This repo's own `__docs__/specs/NNN-*.md` convention is an example of that pattern.
- Drift detection is the obvious unsolved layer. Incumbent SDD tools leave it manual, and the products that address it are small OSS CLIs or a single beta (Augment Intent). There is room for a hosted service that continuously scores spec ↔ code conformance across repos.
- No SDD tool reaches past "code merged" to release, feature-flag or production telemetry. The "→ release → monitoring" half of the proposed dashboard is uncontested in this category.
- Risk: Kiro (AWS), GitHub (Spec Kit + Copilot) and Augment could each add drift and status views cheaply. The defensible angle is cross-tool aggregation (Figma + any spec format + any Git host + any observability stack), not authoring specs.
- Thoughtworks' "documentation sprawl" caution suggests buyers will value tools that prune and validate specs, not tools that generate more of them.

### Gaps
- Not covered by any source found in this session: Linear agents/specs, Factory.ai (Droids/spec mode), Devin (Cognition), Notion AI specs, ChatPRD, Cursor plan mode files, and Claude Code plan files. Background knowledge, unverified here: ChatPRD is a PRD-writing assistant; Cursor and Claude Code plan modes produce ephemeral or local plan Markdown without lifecycle tracking; Factory and Devin are autonomous coding agents with planning steps, not persistent spec stores. Confirm before using.
- GitHub stars or adoption numbers for Spec Kit, OpenSpec and BMAD were not retrieved (sources noted conflicting star counts).
- Kiro 2026 adoption metrics (MAU, paid seats, revenue) were not found. Its GA date is disputed (Nov 2025 vs May 2026).
- Tessl product status, pricing and any 2026 funding are unconfirmed. Its funding timing conflicts across sources.
- Uvik's drift benchmark figures for BMAD and OpenSpec were not retrieved, and its methodology is unverified.

## Q4. Is there evidence of demand for "spec ↔ code ↔ production" linkage in AI-heavy teams?

### Takeaway
There is qualitative demand for spec ↔ code coupling. HN debates centre on specs drifting from code, Thoughtworks calls SDD adoption by teams using coding agents increasing, community drift-detection tools are multiplying, and Augment markets "living specs". Survey evidence specifically for spec ↔ production linkage was not found. DORA 2025 shows near-universal AI use with low trust and higher instability, which indirectly supports the need for verification and traceability, but it does not ask about specs.

### Cited Findings
- Thoughtworks Radar Vol 35: "Teams working with coding agents are increasingly adopting spec-driven development workflows". It also warns that terminology such as "spec-driven development" and "harness engineering" is used inconsistently. — [Thoughtworks Radar Vol 35](https://www.thoughtworks.com/content/dam/thoughtworks/documents/radar/2026/11/tr_technology_radar_vol_35_en.pdf)
- HN skeptic: "Spec-Driven systems are doomed to fail. There's nothing that couples the english language specs you've written with the actual code and behaviour of the system." — [HN item 47422065](https://news.ycombinator.com/item?id=47422065)
- HN threads also include "Ask HN: Are you still using spec driven development?", a "Verified Spec-Driven Development (VSDD)" proposal, and "Show HN: Spec-Driven AI Development". All show active debate about keeping specs truthful. — [Ask HN 46864948](https://news.ycombinator.com/item?id=46864948); [VSDD 47197595](https://news.ycombinator.com/item?id=47197595); [Show HN 46589002](https://news.ycombinator.com/item?id=46589002); [Spec Kit toolkit thread 45798473](https://news.ycombinator.com/item?id=45798473)
- Counter-view on HN: change the spec, not the code, and approve the code the LLM regenerates, so the spec should never diverge. Others note that AI can reverse-engineer specs from code. — [HN 45798473](https://news.ycombinator.com/item?id=45798473)
- Essay: "a specification's usefulness is inversely proportional to the ease with which it can drift from the running system undetected". It argues executable tests are the only specs that reliably stay in sync. — [tddbuddy: Spec-driven is the drift trap](https://www.tddbuddy.com/blog/spec-driven-is-the-drift-trap/); [dev.to version](https://dev.to/tmfrisinger/spec-driven-development-is-the-prose-spec-drift-trap-reborn-4p99)
- HN discussion on how version control will evolve for the agent boom, an adjacent signal about tracking agent-produced changes. — [HN 48844709](https://news.ycombinator.com/item?id=48844709)
- DORA 2025 "State of AI-assisted Software Development" (survey June 13 – July 21, 2025; nearly 5,000 respondents per secondary sources). About 90–95% use AI, about 30% report little or no trust in AI-generated code, and AI "increases throughput" but "also increases instability". Figures vary across secondary summaries. — [DORA 2025 report PDF](https://services.google.com/fh/files/misc/2025_state_of_ai_assisted_software_development.pdf); [Decrypt](https://decrypt.co/341190/coders-now-use-ai-nobody-trusts-it-google-finds); [Faros AI summary](https://faros.ai/blog/key-takeaways-from-the-dora-report-2025)
- Supply-side signals: several open-source drift tools (SpecSync, CorvidLabs spec-sync, spec-kit-sync, oas-drift, a Claude Code drift skill) and explainer content on "spec drift" (SpecStory, TrueSpec) all appeared in the 2025–2026 timeframe. — see Q3 sources

### Inferences
- Demand clearly exists for spec ↔ code integrity among AI-heavy teams. The skeptics' main objection (nothing couples spec to behaviour) is itself the product opportunity.
- Demand for the production link (spec ↔ monitoring) is inferred rather than evidenced. DORA's finding that AI raises instability while trust stays low suggests teams want to know which AI-built feature broke what, but nobody frames this as "spec-level observability" yet. That means customer discovery is needed, and the positioning may need to lead with drift and verification and add production health as the follow-on.
- Thoughtworks' caution on documentation sprawl means a per-feature dashboard should show spec freshness and conformance, not just list specs.

### Gaps
- No survey (DORA, Stack Overflow, JetBrains, State of AI Engineering) was found that asks specifically about specs, requirements traceability, or linking features to production metrics.
- The HN threads were seen only through search snippets, so vote counts, dates and sentiment balance were not measured.
- No customer-interview, Reddit or analyst (Gartner/Forrester) evidence for "feature lifecycle dashboard" demand was collected.
- Adjacent competitors that might already offer feature-level production linkage were outside this scope and not researched: feature-flag platforms (LaunchDarkly, Statsig), engineering intelligence (LinearB, Jellyfish, Faros, Swarmia), and product analytics. The report writer should check other researchers' notes for them.
