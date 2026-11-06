# Gleaner - 2026-11-06-combined

The 2026-11-06 sessions teach how Runvil captures tacit knowledge through stories, how the engine, harness, and plug-ins are separated, and why the virtual file system's determinism drives the open synchronous versus async choice.

- Source: 2026-11-06-prevatte-falco.md (PF), 2026-11-06-class-1.md (C1), 2026-11-06-class-2.md (C2), combined in that order; each stamp is prefixed with its source
- Span: PF 0:09 to C2 53:27
- Chunks: 5
- Items: 137 (59 design points, 16 teachable, 21 principles, 8 open questions, 33 to do)
- Gaps: None

## Design Points

1. **Classroom model** [PF 0:26] - For about two weeks coding pauses and the team works as one classroom with a single shared assignment, instead of individual assignments for each person.
2. **Capture pipeline** [PF 2:53] - Capture interviews a subject with tacit knowledge, saves the transcript, and runs an extraction prompt that turns their stories into experiences, principles, and technical design decisions.
3. **Workflow is the product** [PF 3:21] - The workflow used to extract one person's knowledge through interviews is the same product the company intends to sell, which is why the whole team takes part.
4. **Prototype tools** [PF 3:40] - Prototypes exist for interviewing (Mentographist), teaching (Pedagogue) and transcripts (Talktron [Toctron]); still missing are the pipeline, trained people, prompts and automation infrastructure.
5. **Team huddle** [PF 5:22] - The class alternates instructor sessions with learner-only huddles, where participants review the minutes, compile questions, and compare what worked, free of the instructor's influence.
6. **Weekly prompt showcase** [PF 6:30] - Every week or two a few people show their prompts to the group, so progress is visible and comparable.
7. **Shared Cursor tooling** [PF 6:57] - Everyone gets Cursor in addition to their preferred tools, because the same prompts give the same results there and the team can compare notes.
8. **Transcript repository** [PF 7:14] - Every transcript goes into the repository as markdown with a timestamp and front matter of keywords and topic, so it can be indexed later, as at a client site.
9. **Student prompt folders** [PF 8:37] - The repository has a student folder with one directory per team member holding their prompts, so everyone can see everyone's work.
10. **On-site commercial product** [PF 10:36] - The first product runs as a docker container or cloud service on a client's network, interviews staff, turns transcripts into experiences and principles, and earns license, service, subscription, and support fees.
11. **Autobiographical persona** [PF 11:02] - The second product interviews a person, or those who know them, and yields a trusted AI confidant that answers questions about them, sold by subscription or fixed-cost buyout.
12. **Knowledge marketplace** [PF 12:47] - The third product is a YouTube-like marketplace where creators publish knowledge bases as adaptive learning programs that consumers search and pay for with credits, creators earning a share by popularity.
13. **Pedagogue teacher builder** [PF 13:19] - The Pedagogue is an adaptive learning program that builds a personalized teacher from a knowledge database, remembering and knowing each learner.
14. **Free rating incentive** [PF 14:14] - A random 10 percent of learning programs are free to those who use them, and the user must rate the program afterward, so ratings are guaranteed and cannot be gamed.
15. **Class day structure** [PF 18:27] - The draft day mirrors school: 50-minute class, 10-minute break, individual study, lunch, a second 50-minute class, then a teacher-free breakout.
16. **Prompts run on every transcript** [PF 19:05] - Each transcript is added to the repository and every team member's extraction prompt runs on it, so results can be compared.
17. **UI easiest component** [PF 23:32] - The UI is the easiest component to build because it can copy Visual Studio Code, and the gateway is second easiest by copying an existing project.
18. **Repo naming** [PF 26:33] - The repository is named Runvil Classroom without a date; a later restart can archive the master branch and make a new one, and renaming waits until several repositories exist.
19. **Adaptive tutor rules** [C1 6:53] - The learning-program prompt embeds teaching rules such as never revealing the answer, and backing off to a simpler topic after two or three mistakes in a row to rebuild confidence.
20. **Authority per section** [C1 11:46] - A Runville prompt author decides whether code or the model holds orchestration authority, for the whole prompt or part of it, and may delegate classification to a classifier.
21. **Selective and ad hoc tools** [C1 12:05] - The author can offer the model different tools in different prompt sections and can invent ad hoc tools for the model to call.
22. **Custom context control** [C1 13:47] - The Workshop IDE exists so authors can enrich context their own way and choose per prompt what survives compaction, instead of relying on a vendor's server-side enrichment.
23. **Workshop breakpoints** [C1 14:51] - The Workshop lets an author run a prompt with a keyboard shortcut, set a breakpoint, and inspect the store, files, and variables before changing something and rerunning.
24. **Record and replay** [C1 15:46] - Recording every tool call, Lua function call, and model interaction lets a rerun replay to a breakpoint, so fixing step 7 of a pipeline skips the slow steps 1 through 6.
25. **Harness-delivered tutoring** [C1 27:13] - A Runville harness can connect a learning program to a web server that renders a textbook-style page, with model tool calls triggering quizzes that gate the next section.
26. **Class as prototype** [C1 28:23] - The class itself enacts the capture-and-transfer pipeline, and records of what works for learners become the case study and research backing the Knowledge Capture Corporation product.
27. **Gateway protocol translation** [C1 37:38] - The gateway speaks a generic OpenAI-style inference protocol of messages and tools, and translates it into each frontier vendor's special API so models like Claude can be used fully.
28. **Anthropic signed thinking** [C1 38:52] - Anthropic thinking blocks carry a cryptographic signature, so clients must return them unedited, unremoved, and in order, or the chain breaks, the cache is lost, and the request fails with an HTTP error.
29. **OpenAI encrypted thinking** [C1 39:42] - OpenAI returns encrypted, redacted thinking blocks that must be sent back exactly as received, and any tampering gets the request rejected.
30. **Vendor tool differences** [C1 40:04] - Vendors handle tools in slightly different ways, so the gateway's translation layer must account for those differences as well as the thinking blocks.
31. **Minimal gateway extensions** [C1 40:04] - The gateway endpoint supports the generic API plus just enough extensions that other apps using it are unaffected, while the Runville engine uses minimal extensions the gateway translates into proprietary APIs.
32. **Streamed chat round trip** [C1 41:58] - The agent packs chat history and new input, sends it through the gateway to Claude, and returns the reply as chat, with a streaming extension showing words as they arrive.
33. **Growing message stack** [C1 43:02] - Runville keeps a growing stack of messages, including a system prompt, sends the whole stack to the model each turn, and compacts it when it gets too big.
34. **Chat prompt append** [C1 43:58] - The chat prompt appends each user input to the messages with a builder, then runs the tool loop, which sends the new stack of messages to the model.
35. **Rust-backed messages object** [C1 44:22] - The Lua-side messages object is backed by a Rust struct, which can hold invisible extras that Lua cannot see, such as the tool catalog, for talking to the gateway.
36. **Transcript-only prompts** [C1 46:37] - Extraction prompts read only the transcript, not the code or other sources, so they stay quick to run and easy to reuse.
37. **Required packs** [C2 7:55] - A prompt offers tools through tools.offer and here requires three packs, user input, standard tools on the virtual file system, and web; user input is essential because otherwise nothing can be typed to the model.
38. **Gateway API keys** [C2 9:45] - The gateway loads API keys from an environment file filled through the settings UI and attaches the right one to each request, keeping keys in one process with less attack surface.
39. **Engine versus harness** [C2 13:05] - The engine is the minimal part needed to deliver the Runville language, and the harness is everything needed for it to interact with the outside world; removing any piece of either breaks it.
40. **Tools as plug-ins** [C2 14:21] - User input, web, and standard tools sit outside the harness as plug-ins, so a bare harness can only talk to the model and cannot make tool calls.
41. **Finishable engine** [C2 14:57] - A minimal engine keeps tests finite in scope and lets it be declared finished, so new features go into the harness or plug-ins instead of the engine.
42. **Alternative harnesses** [C2 15:59] - Others can write their own harness for the prompting language, for example a smaller one for a phone with a few built-ins and no plug-ins.
43. **Plug-ins and packs** [C2 17:29] - A plug-in is shared-library code providing zero or more packs, each pack having a name and zero or more tools, and a prompt declaring a pack can run only when its plug-in is available.
44. **Plug-in file access** [C2 18:55] - Plug-in tools must do file IO through the virtual file system to see mounted files such as the store, and the VFS API is deliberately small, so exporting it is not hard.
45. **Access object** [C2 20:10] - The engine gives each plug-in an Access object offering file operations such as append, copy, exists, list, glob, partial read, remove, rename, and write.
46. **Async at tool call** [C2 28:05] - One recommendation is to keep Access members synchronous and make the whole tool call the async boundary; synchronous members can block, so a big grep stalls every tool and limits cancellation and remote back ends.
47. **VFS determinism** [C2 31:50] - The virtual file system orders operations into a claims table of each chain's reads, writes, and checks, and stops a later chain with overlapping claims with a nondeterminism error, so every run behaves identically.
48. **Worktrees versus fan-out** [C2 33:20] - Separate git worktrees per agent do not remove merge risk, because fan-out subagents writing separate virtual files must still be combined, and combination safety depends on the algorithm.
49. **Sync chosen for determinism** [C2 34:04] - The engine is synchronous on purpose to keep determinism, and shipping synchronous first allows plug-ins right away, bringing the bash kit and letting people work separately so the team can scale.
50. **Async API as veneer** [C2 35:39] - Backends are synchronous, so any async API is a veneer that runs the synchronous operation on another thread and wakes a future.
51. **Thread hop placement** [C2 37:47] - Running a tool body on a worker thread means the host must wait for the worker to acknowledge before ending the access, and for Rust-linked packs the thread hop belongs in the host's performer future.
52. **Shared VFS and vended access** [C2 38:27] - All prompts share one virtual file system through an Arc-like reference, and nothing can be done with it directly; a caller must vend an access object first.
53. **Access carries chain ID** [C2 38:53] - Each access embeds its chain's identifier, which lets the claims table track who is doing what across prompts and stop two prompts that touch the same file.
54. **Access as borrow** [C2 39:31] - Giving plug-ins access as a borrow means a plug-in cannot hand it to anyone else or create its own, which allows stronger correctness guarantees.
55. **Ending access releases claims** [C2 40:10] - When a plug-in returns or drops its access, all its claims on files are released, so those files can be touched by others.
56. **Revoke access on cancel** [C2 40:44] - On cancel the host revokes a plug-in's access instead of calling into it, so stop and app exit are immediate; the plug-in handles the revoked error on its next use.
57. **Inference dominates VFS time** [C2 41:38] - Model inference takes orders of magnitude longer than synchronous VFS operations, which weakens the performance case for making file operations async.
58. **Cost of going async** [C2 43:00] - Async brings in-flight work after abort, because a blocking synchronous call cannot be canceled just by dropping a future.
59. **Sync simplicity tradeoff** [C2 45:00] - Synchronous keeps return types simple and needs no new data structures, so plug-ins are easy to build, with the drawback that it can tie up a Tokio [Tokyo] thread.

## Teachable Material

60. **Tacit knowledge** [PF 3:55] - Tacit knowledge is experience and judgment gained from long practice with no written rules, and interviews where workers tell stories can capture it, such as how a leader made every product a hit.
61. **Adaptive learning** [PF 13:14] - Adaptive learning works like adaptive testing, except the person is learning instead of being quizzed, though they still get quizzed.
62. **Tacit knowledge recall** [PF 24:05] - Asking someone to tell everything they know fails because knowledge is contextual, while a specific question such as how a tool call was designed brings the memory back.
63. **Stories as the key** [PF 25:10] - People remember stories and are happy to tell them, which is the key insight behind building the interview approach on storytelling.
64. **Agent harness tutorial** [C1 4:09] - Understanding one generic agent harness explains how Cursor, Claude Code [Clog code], and ChatGPT [Chat GBT] web work, including the parts a Runville prompt relies on.
65. **Succession problem** [C1 26:12] - When someone who holds knowledge leaves, it leaves with them and their replacement cannot be trained, so capturing it and transferring it to another person solves succession.
66. **Interviewer story guidance** [C1 34:15] - In the SAGE paper's premise, one person holds the knowledge and an interviewer skillfully guides them toward a subject and gets them to tell a story.
67. **Story walk flow** [C1 34:46] - Mentograph chats until it finds a story to lock onto, then runs a story walk of questions about it, and moves to variations once the story's energy runs down.
68. **Story variations** [C1 35:16] - Variations ask what-if questions, such as a different choice, through seven kinds of story modification, and the resulting exploration reveals how the person thinks, making it the most valuable capture.
69. **Research reports for learning** [C1 37:02] - Prompting an agent to search the web and write a report on a topic is a way to learn an unfamiliar area, such as how agent harnesses work.
70. **Tools and catalog** [C2 5:06] - A model alone can only return text, so it needs tools, each defined like a function with a name, description, and parameters, and the tool catalog is sent with the conversation.
71. **Tool count limits** [C2 6:02] - Larger models handle more tools, but offering too many, such as 500, confuses any model, and small models of roughly 3 to 9 billion parameters manage only about 3 to 8 tools.
72. **Web search cost** [C2 8:47] - Search engines such as Google reject bots, so agent web search needs a paid API key from a provider such as Brave.
73. **Ask mode** [C2 24:09] - Switching the AI assistant to ask mode while discussing design stops it from getting excited and starting to implement changes.
74. **STE 100** [C2 31:06] - STE 100, a controlled technical English from flight manuals, was suggested as a way to make AI output plainer, though the jargon at issue here is specific to the virtual file system.
75. **Access as key card** [C2 46:13] - An access works like a key card: every use checks the rules, records claims like sticky notes, tells other chains about them, and only then performs the action.

## Principles

76. **Self-assigned work** [PF 1:33] - Team members should assign themselves work and do it on their own, without anyone having to tell them what to do.
77. **Work as a team** [PF 5:36] - The team grows more efficiently when people work together on shared work than when each person works alone in a separate repository.
78. **Compare prompts to explore** [PF 8:55] - Compare everyone's prompts to discover what works best, as exploration in unfamiliar territory rather than competition.
79. **Knowledgeable confidant, not clone** [PF 11:14] - A persona built from a person should be a knowledgeable, trusted confidant, not a virtual clone, because a clone falls into the uncanny valley and nobody wants that.
80. **Ideas versus execution** [PF 15:16] - Ideas are plentiful and people who can execute on them are scarce, so sharing an idea early carries little risk of it being copied.
81. **Polish to perfection** [PF 16:27] - Without quarterly earnings or shareholder calls, work can be polished until it is perfect before shipping, rather than settling for good enough.
82. **Try then adjust** [PF 21:01] - When the best schedule cannot be known ahead of time, try one, keep it if it works, and adjust it if it does not.
83. **Record sessions** [C1 17:20] - Record every session as a transcript, clean it up, turn it into minutes, and mine it later to create documentation for the system.
84. **Undocumented design decisions** [C1 18:39] - Design decisions that exist only in one person's head leave nobody able to tell whether code is correct or an implementation is missing, so they must be extracted.
85. **Extract knowledge, not summary** [C1 19:46] - A summary of what was said is not knowledge, so extraction prompts should pull out teachable content rather than restate statements or events.
86. **Self-merge to move fast** [C1 20:59] - Everyone has write access and merges their own pull requests so work moves quickly and no one waits on review.
87. **Write prompts by hand** [C1 23:45] - Prompts must be written by hand and looked up rather than generated with AI, because using AI means the author learns nothing about how they work.
88. **Gaps guide priorities** [C1 24:13] - Missing Runville features that authors hit while converting prompts, or their workarounds, steer which features get prioritized next.
89. **Focus on interesting problems** [C1 32:10] - Spend team effort on the novel, interesting part of a product and leave problems already solved many times, such as cloud deployment, to partners who specialize in them.
90. **Newer reports are better** [C1 36:13] - In a repository of accumulated reports, newer material is more reliable than older material, so recent files should be trusted over history.
91. **Capture tacit knowledge** [C1 40:59] - The knowledge worth extracting is tacit knowledge that reading the Rust code will not reveal and would only overwhelm a newcomer.
92. **Small isolated components** [C2 13:05] - Agentic workflows should keep components small and isolated, because large interconnected pieces confuse the model and are harder to audit.
93. **Hard-to-reverse choices** [C2 22:09] - Take extra care with hard-to-reverse choices, such as changing every function in an API, which forces rewriting tools and tests, unlike an easily changed value like a max concurrency setting.
94. **Jargon blocks evaluation** [C2 30:22] - An AI recommendation cannot be properly evaluated by someone who does not know its jargon, so unexplained terms such as blocking pool undermine the review.
95. **Understand before approving** [C2 36:51] - A consequential design decision should not be made casually; understand the generated rationale before approving it.
96. **Re-explain without jargon** [C2 44:06] - When a model's explanation uses terms you cannot evaluate, ask it to explain again in more detail, with no jargon, for someone just learning the system.

## Open Questions

97. **Monday newcomer integration** [PF 21:39] - How should the three new team members who missed the first session join on Monday, given the new QA hire may sit in but works separately from engineering?
98. **Knowing what to do** [PF 23:24] - Once people are placed on their assigned parts of Runvil, how do they learn what to do within those parts?
99. **Transcript to teachable form** [C1 20:59] - What to extract from an interview transcript, and how to store and present it so people can learn it, is undecided and remains the core unsolved problem.
100. **Judging extraction prompts** [C1 45:48] - What evaluation criteria show whether a transcript-extraction prompt did a good job is undecided, to be settled by comparing results together and commenting on what looks good.
101. **Where keys live** [C2 10:35] - Should web search keys live in the gateway or the harness, given shared organization keys on a server favor the gateway while individual keys may favor the harness?
102. **Sync or async VFS** [C2 21:00] - Whether the plug-in virtual file system API should be synchronous, asynchronous, or both, with async possibly added later as a second surface, is undecided, and reversing the choice is expensive.
103. **Network-bound plug-ins** [C2 29:22] - Whether most plug-ins will be network bound is unknown, and the answer would decide whether to revisit the recommendation to keep access synchronous.
104. **Long blocking tools** [C2 42:33] - How a synchronous VFS should handle a long-running tool, such as one that zips a large directory, without blocking all other prompts is left open.

## To Do

105. **Create classroom repository** [PF 1:11] - Create the private repository that saves every transcript, named Runvil Classroom (owner: Vinnie)
106. **First class content** [PF 2:21] - Use the first class to explain the vision for Runvil and the Knowledge Capture Corporation, then how the team will interview and work (owner: Vinnie)
107. **Cursor accounts** [PF 6:36] - Make sure everyone has a Cursor account, the $200 a month plan or the $20 plan where it is not their primary tool (owner not stated)
108. **Extraction prompts** [PF 8:19] - Each person writes an extraction prompt in their own student folder that reviews a transcript and outputs a markdown file, runs it on the transcript, saves the output, and submits a pull request (owner: every team member)
109. **Lesson plan document** [PF 9:44] - Turn the transcript into a lesson plan document with immediate steps (owner: Cynthia)
110. **Class signups** [PF 18:02] - Set up signups and make sure everyone is synchronized on class times (owner not stated)
111. **Scribe tool** [PF 19:31] - Each person makes a scribe tool that adds a transcript to the repository and runs their prompts, so results can be compared (owner: every team member)
112. **Schedule draft** [PF 21:26] - Draft the class schedule, send it for approval, then post it in the team channel (owner: Cynthia)
113. **Fix and file transcript** [PF 28:27] - Fix this meeting's transcript and put it in the repository so minutes can be extracted from it (owner: Vinnie)
114. **Document Cursor story** [C1 13:30] - Put the story of why the author wants his own IDE instead of Cursor into a document from the transcript (owner not stated)
115. **Transcript-to-principles prompt** [C1 20:36] - Write a prompt that turns the transcript into design principles, refine it without overspending time, and merge it by pull request into your own directory (owner: each class participant)
116. **Provide design principles** [C1 20:36] - Give participants design principles so they can test and adjust their extraction prompts (owner: Vinnie)
117. **Convert public prompts** [C1 21:48] - Each participant chooses one existing public tool prompt to convert into a Runville prompt, written by hand in small weekly steps; two people may share a prompt (owner: each class participant)
118. **Preview Agora output** [C1 22:08] - Show newer participants the output page of the Agora project to illustrate what Runville can do and the kind of products planned (owner not stated)
119. **Find extra prompts** [C1 23:45] - Find another prompt or alternative task for a participant who cannot get a prompt because too few remain (owner: Vinnie)
120. **Gateway vendor translation** [C1 38:15] - Implement in the gateway the differences between the generic protocol and vendor APIs, as documented in the research file, working with the Runville engine (owner: Andrew Wheeler)
121. **Share the transcript** [C1 45:15] - Put the meeting transcript where everyone can access it so each person can run their prompt on it (owner: Cynthia)
122. **Run scribe comparison** [C1 46:06] - Run scribe on the transcript so people can compare its extraction against their own prompts' output (owner: Vinnie)
123. **Mentograph assignment** [C1 47:11] - Work on the Mentograph [entograph] prompt (owner: Will)
124. **Agora assignment** [C1 47:11] - Work on the Agora prompt (owner: Jason and Glenn)
125. **Document incomplete areas** [C2 16:25] - Organize the incomplete aspects of Runville into a document and commit it to the repo (owner: Cynthia)
126. **Build plug-in loading** [C2 17:29] - Build real plug-in loading by producing DLLs, scanning a directory to load them, and letting plug-ins register packs (owner not stated)
127. **Export VFS to plug-ins** [C2 19:17] - Export the virtual file system API to plug-ins so their tools can see the store and mounted virtual files (owner not stated)
128. **Weigh in on sync or async** [C2 21:32] - Give input on whether the plug-in virtual file system API should be synchronous or asynchronous, especially from people who know Tokio [Tokyo] well (owner: everyone, especially those who know Tokio)
129. **Study the prompt output** [C2 43:46] - Hold a separate team session, run the same prompts, and ask the model what the unclear terms mean (owner: the students)
130. **Document VFS crate** [C2 44:06] - Document the VFS crate so its design is easier to understand (owner not stated)
131. **Transcript parsing prompt** [C2 47:28] - Finish the transcript parsing and summarizing prompt, which was started but not completed (owner: Greg)
132. **Use forks for PRs** [C2 47:48] - Create a fork of the repo and open pull requests from it, rather than pushing topical branches to the main repo, matching the pattern used across the team's repos (owner: Mateo Robles)
133. **Fix Runvil Classroom repo** [C2 48:18] - Fix the Runvil Classroom repo setup, which needs admin rights to configure the required role (owner: Greg)
134. **Ask for next goals** [C2 50:35] - At the end of Monday's meeting, ask what the group should accomplish before the next one (owner: Cynthia)
135. **Read the course material** [C2 50:53] - Read the reading material between sessions, treating the course like school: read, ask questions, and take notes (owner: the students)
136. **Publish minutes** [C2 52:07] - Run the scribe tool on the single raw transcript and put the minutes up (owner: Vinnie)
137. **Run extraction prompt** [C2 52:07] - Run the extraction prompt on the raw transcript and put the output in the student folder (owner: each student, especially Mateo Robles)

*2026-11-06 15:58 - Claude Sonnet 5.5*
