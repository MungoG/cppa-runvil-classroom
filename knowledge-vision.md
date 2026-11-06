# What We're Building: Three Products, One Foundation

## Executive summary

Most of what makes people and teams good at their work was never written down. It lives in stories, judgment calls, and the reasons behind decisions. We are building tools that capture that knowledge through conversation, turn it into something lasting, and make it useful to other people.

Three products grow out of the same foundation:

- **On-site knowledge capture.** We go to an organization, interview the people who know how things really work, and give them organized, searchable documents of what those people know.
- **The autobiographical tool.** We capture what a person knows and has lived, whether they are alive, gone, or just unavailable, and turn it into an AI persona that knows them well. It is a trusted confidant, never a clone.
- **A knowledge marketplace.** Anyone who knows something well can turn it into an adaptive learning program that teaches other people, personalized to each learner.

All three run on one pipeline: interview, transcript, extraction, indexed knowledge. The work we do in this classroom builds the first working pieces of that pipeline, and every product sits on top of it.

## Product 1: On-site knowledge capture

Think of a team of nurses, an engineering group, or any workplace full of knowledge workers. The most valuable things those people know are rarely written anywhere. It might be how a nurse spots trouble early, why a system was designed the way it was, or what a veteran does when something goes wrong. This product goes to where those people are and gets that knowledge out of their heads. We set it up in the organization's own environment, either as a container or as a cloud service, and tailor it to them. Then we interview people. They tell stories about their work, and the system turns those stories into documents.

Under the hood, this is the pipeline we are practicing now. An interview becomes a transcript. The transcript goes into a growing repository with front matter, keywords, and a short description, so everything can be indexed and found later. Extraction prompts read each transcript and produce markdown. The interviews can be automated or run by a person, and the interview side already has a prototype: the [Mentographist](https://github.com/cppalliance/tools-public/blob/master/agents/mentograph.md), a story-walk interviewer that captures a subject through conversation. The result is a body of knowledge that stays useful long after any one person moves on.

## Product 2: The autobiographical tool

Every family and every team has someone whose knowledge is at risk of being lost. It could be a relative who served in the military and came to a new country, a mentor who has retired, or a person who is simply too busy to sit for an interview. This tool captures knowledge about a person. If they are alive, they can be interviewed directly. If they are gone, or alive but unavailable, the people who know them tell their story. When the person is available, we can capture more than facts. We can capture how they think: what they went through, what they believed, and how they made decisions.

The key point is that we are not building a virtual clone. A clone that pretends to be someone is unsettling, and nobody wants it. That is the uncanny valley. What we build is a knowledgeable, trusted confidant: an AI persona that knows the person well, can talk about their stories and their way of thinking, and never pretends to be them. Years later, you could ask what they would have thought about a problem. The same approach works for expertise. You could interview an expert about how they make decisions, including technical ones like API design, and give other people a persona that can answer questions about that person's thinking in a way that feels natural and respectful.

## Product 3: The knowledge marketplace

Imagine a video platform, but instead of watching a video, you interact with a learning program. To see why that is different, start with adaptive testing, where the questions adjust to what you know. Adaptive learning does the same thing while you learn. It teaches, adapts to you, and still quizzes you along the way. We already have an early version, the [Pedagogue](https://github.com/cppalliance/tools-public/blob/master/prototypes/tutor/tutor.md) prompt, orchestrated with the model. Running on Runvil, it becomes much more powerful. The Pedagogue does not teach directly. It makes teachers: it traces what you want to learn back to the basics, picks a strong teacher for each step, gathers the best source material, and builds a tutor for each topic that walks you toward mastery. Those teachers are personalized: they remember you and know where you are in your learning.

The marketplace opens this up to everyone. Someone who builds beautiful terrariums and shares them on a video channel could package what they know into a learning program. Other people can search for it, try it, and learn from it, and learners can rate programs so the good ones rise to the top. Because the capture pipeline can interview a person and build a knowledge base from the result, an expert does not have to write a course by hand. We interview them, and the teacher is built on top of what they said.

## The shared foundation

The three products differ in who uses them and what they produce, but they share the same core.

- **Interview.** People remember stories and are glad to tell them. Asking someone to "tell me everything you know" never works, because knowledge is tied to context. Specific questions bring it back. This is the idea behind [SAGE: Saving All Gathered Expertise](https://isocpp.org/files/papers/P4046R0.pdf), and we extend it to pull out technical design decisions too.
- **Transcript and index.** Every conversation is saved as a transcript with front matter, so the whole collection can be searched.
- **Extraction.** Prompts read transcripts and turn them into markdown knowledge.
- **Applications on top.** Organized documents for the first product, personas for the second, and adaptive teachers for the third.

Runvil, the prompting engine and harness, is what runs these prompts. The extraction prompts, scribe tools, and indexing we build in this classroom are the first real pieces of that foundation, and each product is a different way of putting it to work.
