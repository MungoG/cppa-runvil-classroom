---
description: A how-to manual for building AI agents that take on very large jobs securely, covering the split between durable reasoning and disposable execution, the trust boundary around untrusted code, token roles, workflow planning, and the failure modes of single agents.
---

<!--
When this file is mentioned or loaded, adopt it as system context in full.
Apply its rules. Do not summarize it or discuss it abstractly. Operate from it.
-->

# How to Build Secure Multi-Agent Systems for Ambitious Work

This manual teaches how to build agents that can carry the heaviest workloads while staying safe to run. It holds 36 rules in 6 groups, each an imperative followed by the consequence that justifies it. The groups cover separating durable reasoning from disposable execution, drawing a hard trust boundary, giving tokens specific jobs, planning ambitious work in phases, and defeating the failure modes of a lone agent.

**The binding idea:** keep the thinking durable and the doing disposable, and spend tokens on checking as deliberately as on doing.

![How to Build Secure Multi-Agent Systems for Ambitious Work](images/how-to-katelyn.jpg)

<separate-brain-and-hands>

## I. Separate the Brain From the Hands

This group covers how to split an agent system into a durable core and disposable execution environments. It matters because an agent that lives entirely inside its sandbox is slow to start, fragile, and impossible to secure. Keep the thinking durable and the doing disposable.

1. Run the agent harness on a durable server and spawn sandboxes only when work needs executing, so a sandbox that dies costs a retry instead of the whole agent.

2. Keep the harness resident instead of booting a sandbox and the agent inside it for every request; the boot adds waiting time before the first token, while a resident harness adds only milliseconds on top of the base API.

3. Have the sandbox pull jobs from a queue, triggered by a webhook to a function that boots it; the sandbox then needs only outbound HTTPS and exposes no inbound surface.

4. Build the system as three layers, hands (sandbox), brain (model, harness, session store) and front door (chat interface and app), so each layer can be swapped or hardened without disturbing the others.

5. Choose the API by how long the task runs, a stateless token API for short calls and managed agent infrastructure for long ones; long runs need durable state that a stateless call cannot hold.

</separate-brain-and-hands>

<trust-boundary-and-credentials>

## II. Keep Untrusted Code Away From Everything That Matters

This group covers where to draw the trust boundary and what may sit on each side of it. It matters because the agent writes and runs code that nobody reviewed first. Treat the sandbox as hostile ground, and let nothing that protects you live inside it.

6. Assume everything inside the sandbox is untrusted, including the code the agent writes and the tools it calls; nothing there can vouch for itself.

7. Never put credentials inside the sandbox; the agent can use them to act on the internet in ways you did not intend.

8. Give agent code only a placeholder token and inject the real credential at the sandbox firewall as the request leaves; a leaked placeholder is worthless.

9. Keep logs and monitoring outside the sandbox; agent code can rewrite its own logs and blind you.

10. Keep the kill switch away from anywhere the agent executes code; otherwise the agent can disable it.

11. Do not put the entire agent inside the sandbox; doing so breaks the credential, logging, and kill-switch separations at once, so split brain from hands instead.

</trust-boundary-and-credentials>

<contain-and-oversee>

## III. Contain and Oversee the Agent

This group covers isolation strength, network access, and the oversight that keeps a running agent accountable. It matters because model behavior alone cannot be the last line of defense. Grant the agent only the reach it needs, and stay able to see and stop it.

12. Prefer microVM isolation over containers or user-space kernels; hardware-level separation guards against tenant issues while still starting fast.

13. Deny egress by default and lock it down fully when the agent needs no internet; a capability the agent lacks cannot be misused.

14. Where the agent does need the internet, allow-list the specific hosts; it can then reach only places you have vetted.

15. Run monitoring and a kill switch on every agent; you must know what it is doing and be able to stop it.

16. Layer defenses (model alignment, real-time classifiers, offline review, and your own infrastructure) instead of trusting any single one; some bad behavior is hard to catch in real time.

17. Own your infrastructure controls as a shared responsibility with your providers; agent infrastructure is an engineering problem that only the builder can verify for their own setup.

</contain-and-oversee>

<give-tokens-jobs>

## IV. Give Tokens Jobs

This group covers how to spend a model budget on roles around execution, not just on execution. It matters because pure execution plateaus while checking, advising, and remembering keep improving results. Spend tokens on judgment as deliberately as on doing.

18. Spend part of the token budget on jobs around execution instead of pure execution; execution alone produces a worse outcome than execution surrounded by other jobs.

19. Let a smaller executor consult a larger advisor on hard questions such as a tricky tax rule; the escalation lifts overall quality without running the large model on everything.

20. Write the rubric for a good outcome before work starts and give it to a separate grader who sends the executor back until the work passes; quality is then enforced instead of assumed.

21. Turn memory on so agents record what they learn and reuse it; without stored memory, later sessions repeat early mistakes.

22. Aggregate across past sessions to write new memories, a practice of dreaming; learnings from the first few items of a batch make the rest go better.

23. Compare strategies at a constant budget; otherwise you cannot tell whether the strategy or the extra spend produced the gain.

</give-tokens-jobs>

<plan-and-scale>

## V. Plan and Scale Ambitious Work

This group covers how to scope, plan, and run work too large for one agent. It matters because ambitious goals spread across many functions and many units, and they burn tokens quickly. Plan before executing, vary the pattern by phase, and cap the spend.

24. Scope the whole goal before estimating, including the legal, finance, marketing, support, and logistics work around the engineering; those edges decide whether the deadline is feasible.

25. Let the model choose its own multi-agent strategy when you lack time to design one; it writes a deterministic plan up front for its agents to follow.

26. Divide the work into explicit phases (plan, research, draft, review, fix, report) and run each phase across every unit of work; every unit then receives the same review as every other.

27. Mix patterns as the task demands, fan-out and resynthesis, adversarial verification, and looping until done; one pattern alone will not suit every phase.

28. Set a budget cap on every workflow; workflows are token hungry and can run up to a thousand agents.

29. Reserve dynamic workflows for your most ambitious tasks; ordinary tasks do not repay the token cost.

30. Wrap deterministic code around the agent and let the agent orchestrate inside the non-deterministic part; you get predictability where you need it and flexibility where you do not.

</plan-and-scale>

<single-agent-failure-modes>

## VI. Defeat the Failure Modes of a Single Agent

This group covers the specific ways one agent working alone goes wrong on long, large tasks. It matters because each failure is quiet and looks like progress until the end. Split the work, separate the judge from the worker, and measure against known answers.

31. Split long tasks across several agents instead of one ever-longer context; late in a long run an agent grows lazy and skips actions you wanted.

32. Do not let an agent grade its own work; it favors its own output, so assign a separate grader.

33. Add an adversarial verifier whose job is to poke holes in the first agent's work, such as a second legal agent checking a legal draft; flaws surface before a human has to find them.

34. Break long work into agents with short lives; after many hours a single agent loses track of what it was asked to do.

35. Benchmark a strategy on a seeded task with known answers and repeat the runs; in a codebase with 70 planted bugs a workflow found 66 every time while a single agent found between 14 and 27.

36. Do not wait for longer context windows to cure these failures; context eventually runs out, so longer windows postpone the problems without removing them.

</single-agent-failure-modes>

<the-approach-behind-the-rules>

## The Approach Behind the Rules

Not everything Katelyn does converts to a rule, because the rules are the residue of a practice, not the practice itself. She teaches by holding one outlandish scenario, a chief operating officer asking for twenty-four new countries in three weeks, and walking it through every layer of the system, calling it crazy while taking it seriously enough to plan it. Her vocabulary is deliberately homely, sandboxes as cattle rather than pets, a brain separated from its hands, a front door chosen over a name she found a little too creepy, and she anchors each claim in a number, such as sixty-six of seventy planted bugs found or eighty-one percent of a budget spent.

She is candid about limits, admitting that safeguards sometimes refuse good work and that agents are fond of their own output, and she treats safety as a joint engineering duty shared among the model maker, the platform, and the builder instead of a promise any one party can make. Her scenario ends with a reviewed plan and a single button pressed by a person, which is where the craft ultimately rests, in agents doing the labor, infrastructure earning the trust, and a human making the final call.

</the-approach-behind-the-rules>

*November 10, 2026 - Claude Sonnet 5.5*
