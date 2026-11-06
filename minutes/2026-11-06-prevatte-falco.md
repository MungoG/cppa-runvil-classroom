# Runvil Classroom Planning - 2026-11-06

- Meeting: planning conversation for the Runvil classroom
- Date: 2026-11-06
- Format and location: not stated in the transcript (recorded call, about 29 minutes)
- Chairs: none stated

## Attendees

- Vinnie Falco (VF) - instructor
- Cynthia Prevatte (CY) - TA

## Summary

Two participants planned a two-week classroom for the team, built around transcripts of the instructor's sessions and the knowledge capture workflow behind Runvil. They agreed to try 50-minute sessions with a daily schedule that includes a team breakout, named the repository Runvil Classroom, and left follow-up tasks for the instructor and the TA.

### 1. Classroom Concept and First-Class Plan

The instructor proposed a shared two-week classroom in place of individual assignments, and the TA accepted the TA role.

- The team works on one assignment together, and people are expected to take on the work themselves and to work in small groups.
- Each session is transcribed. One or two people organize each transcript into a document, and every transcript goes into a private repository.
- The first class covers the vision for Runvil and the Knowledge Capture Corporation, then how the class works: team members interview the instructor, the instructor tells stories, and AI turns the stories into experiences and principles (the Sage method). The workflow now also extracts technical design decisions.
- The workflow used to extract the instructor's knowledge is the product the team intends to sell. Prototypes exist for interviewing, teaching, and the Sage method. The pipeline, trained people, prompts, and automation infrastructure do not.
- The target use is on-site work with company staff, capturing what knowledge workers know that is not written down.
- The team gets a half-hour huddle without the instructor to review the output and write the next questions for him.
- The instructor wants daily results in the repository.
- Action: the instructor creates the repository.

### 2. Topics, Prompts, and Tooling

The instructor listed the class topics and asked every person to build an extraction prompt, developed in a per-person directory in the repository.

- Topics: knowledge capture, the motivation for Runvil and the workshop, the Knowledge Capture Corporation and the products built on Runvil, the class format, and a prompt for each person.
- Each person writes an extraction prompt that reads a transcript and outputs a markdown file, and every prompt runs on every transcript so the team can compare results.
- Every transcript is added to the repository with a timestamp and front matter (keywords and subject) so it can be indexed later. The instructor said this mirrors what the team will do at a customer site.
- A web app built by a colleague is the interview tool that produces transcripts. It will be used on site.
- The instructor wants everyone on a Cursor account, a $200 per month plan, with a lower-cost plan for people who use it as a secondary tool. People keep their preferred tools and gain access to the same tools so everyone can compare on one system.
- Every week or two, people show their prompts.
- Action: the TA turns the transcript into a lesson plan.

### 3. The Three Planned Products

The instructor described three products: an on-site knowledge capture service, an autobiographical knowledge tool, and a knowledge marketplace.

- On-site service: a Docker container or cloud service customized for the customer, with revenue from service and license fees plus ongoing support.
- Autobiographical tool: captures knowledge about a person, living or dead, from interviews with the person or with people who know them. The result is a knowledgeable, trusted confidant, an AI persona that knows the person. It is explicitly not a virtual clone. Revenue is a subscription, or a one-time purchase that the customer hosts.
- Marketplace: creators publish adaptive learning programs built from knowledge bases, earn a revenue share, and users pay in credits. About 10 percent of interactions are randomly free, and rating a program earns another chance at a free one.
- The adaptive learning program builds personalized teachers from a knowledge database and remembers each user.
- One participant noted that individuals could use the tool to interview relatives.

### 4. Idea Protection and Positioning

One participant asked whether sharing the for-profit plans this early carries risk, and the instructor said he was not worried.

- The instructor holds that ideas are plentiful and people who can execute on them are scarce.
- He has shared his prompts openly, and he said people who saw them did not understand what they were looking at.
- He said the prompts and skills Anthropic publishes have problems, and that Anthropic does not review its published skills with its own model.
- He said that working without shareholder or quarterly pressure lets him polish work to a personal standard, which he said for-profit companies cannot do.
- He does not expect anyone to copy the idea, because he finds it unusual enough that people react with disbelief.

### 5. Class Format and Daily Schedule

The participants agreed to try a daily schedule built on 50-minute sessions and a team breakout, and to adjust it after trying it.

- Sessions run 50 minutes. An hour or more is too long.
- The instructor's version: session, 10-minute break, an hour of individual study, lunch, a second 50-minute session, a 15-minute break, then a team breakout without the instructor.
- The TA's version: a morning 50-minute session, a team breakout, lunch, then an afternoon 50-minute session. The instructor agreed to try it.
- After each transcript, someone adds it to the repository and everyone's prompt runs on it. Each person also builds a scribe tool and the team compares the results.
- All homework lives in the repository. Each week, two or three people show how their prompts have progressed.
- The team needs a schedule and signups so everyone is synchronized. No owner was named.
- Action: the TA prepares a document, sends it to the instructor for approval, then posts it in the channel.

### 6. New Team Members

The TA asked how three new team members will join on Monday after missing the first session, and the room set no integration plan.

- A new QA team member may sit in on sessions as much as he wants.
- The instructor said the QA role is separate from the engineering side. The TA wants the QA member to understand Runvil early.

### 7. Workload, Ownership, and Tacit Knowledge

The instructor said the workload is larger than he expected, and the room identified giving each person clear direction on their part as the hard problem.

- The instructor said the division of work is already decided. The open problem is that people do not yet know what to do in their areas.
- The UI and the gateway are the easiest parts, because they can be modeled on existing products. The rest is harder.
- The classes exist so the team can understand Runvil. The instructor forgets design choices once he makes them.
- Asking someone to "tell me everything you know" does not work. Specific questions, such as how the tool calls were designed, bring the knowledge back.
- People remember stories and are glad to tell them. This is the insight behind the Sage paper, and the instructor asked the TA to read it.
- The TA said her own memory works the same way.

### 8. Repository Setup and Naming

The repository is named Runvil Classroom, with no date in the name.

- The TA asked for the date in the name so a later restart of the classroom could be told apart. The instructor proposed archiving and renaming the master branch for a restart, and renaming the repository later if more than one exists.
- The instructor creates the repository, fixes the transcript, and adds it. The TA then produces the minutes.
- The team meets in about two hours.
- Action: the instructor fixes the transcript and adds it to the repository.
- Action: the TA produces minutes from the transcript.

## Open Questions

- How will the three new team members catch up on the first session when they join on Monday?
- Who owns the class schedule and the signups?
- Which breakout order works better: after the morning session, or after the afternoon session? The room agreed to try one and adjust.
- How will people get direction on their assigned parts of Runvil beyond the classes?
- How will Cursor accounts be provided, and which plan will each person use?

## Discussion

### 1. Classroom Concept and First-Class Plan

Cynthia (CY): There.

Vinnie Falco (VF): Yes, it's all coming clear now.

CY: I was putting together some thoughts and suggestions, but they're irrelevant now. How do you even do that? [AMB-1]

VF: I don't know. My idea is to make a classroom. Forget the coding. I want everyone to put the pencils down and just have a classroom for about two weeks. Do you know about the Knowledge Capture Corporation?

CY: I don't. I saw it for the fruit [?]. I first saw it when you were talking about it, something to do with Harry, yes? [AMB-2]

VF: Sure. Mary. [AMB-2] Here is how I want to work. I talk, and when we make the transcript, you act as my TA.

CY: OK.

VF: Everyone does the work. They take the transcript, maybe two people, and organize it into a document. We make a private repository and save every single transcript. I want the team to assign themselves the work without being told, like little elves getting Santa's sleigh ready.

VF: Hold on a second.

VF: For the first class, I want to explain my vision for Runvil and the Knowledge Capture Corporation. Then I want to explain how I want to work. They interview me, as part of Sage. I tell stories, and we use AI to turn those stories into experiences and principles. We will modify that workflow, because we also need to extract technical design decisions. I have made many technical design decisions in Runvil that we need to extract from my head.

CY: Right.

VF: This is why I want everybody involved. The workflow we use to extract my knowledge is the product we intend to sell.

CY: Oh, OK.

VF: I have been blocked on getting the Knowledge Capture Corporation started. We have prototypes: the Mentographist is an interview tool, and the Pedagogue is a teaching tool. We have Sage and the example interview. I do not have the pipeline, the trained people, the prompts, or the infrastructure to automate it. The goal is to go on site to a company whose people are knowledge workers, or nurses, or whatever. We interview them, they relate their stories and experiences at work, and we extract what they know that is not written down. That information can be passed on. If we could have interviewed Steve Jobs, he would have told us how he made sure every iPhone was a hit, and we could have captured it.

VF: I want to see daily results in this repository. I will make the new repository. There are no more individuals with separate assignments. We are a classroom now, with one assignment, and everyone is responsible for it. People will often have to work together. I want them to have their own half-hour huddle away from me, where they go over the output of the minutes and put together the list of questions they want to ask me next.

VF: I want them to work with each other. I don't want people working individually in separate repositories, one talking to Cynthia and another talking to me. That is not an efficient way to grow the team and get everyone where they need to be.

CY: I agree one hundred percent. This is exciting.

### 2. Topics, Prompts, and Tooling

VF: Thanks. The topics I want to discuss are knowledge capture, my motivation for Runvil and the workshop, the Knowledge Capture Corporation and the products I want to build on Runvil, and the format for these classes. Everyone is going to build a prompt, and every week or two everyone shows their prompt.

VF: I want people to write prompts. It would be good to make sure everyone has one of the $200 a month Cursor accounts, or the 20 [AMB-4] if Cursor is not their primary tool. We can start them at 20 and move up. Some people use pi [?] or z [?]. If that works for them, that is fine. I think we should give everyone additional access to the same tools, not to replace what they use, but so we can all compare notes on the same system. These prompts work better when they run in Cursor, so everyone gets the same result to compare.

CY: I think it would be better if they all worked in Cursor.

VF: The format is that I talk, tell stories, or take questions for an hour. We take that transcript, and one or two people scribe it. First they add it to the repository with the timestamp and front matter, with keywords and what it is about, so we can index it later. That is crucial, because it is what we will do when we deploy this at a company.

VF: We will give them a cloud application, a web app, like the one Will just built, the Talktron. It is the interview tool, and it creates the transcript. On site we interview people, automated or manual, the tool creates the transcript, and it goes into the growing repository. We are enacting that here.

CY: Yes, one hundred percent.

VF: Then they take the transcript and extract the knowledge. I want every person to create an extraction prompt, a prompt that goes over the transcript and outputs a markdown file. The repository will have a student folder with a directory for each team member, where they put their prompts so we can all see everyone's work.

VF: Then we will see whose prompts work better. It is not a competition. We are exploring, and we will discover what works best for us. I am in uncharted territory. This is the first time in my life I have produced 500,000 lines of code using AI, so I am unsure what to do with it.

VF: I can learn Rust anytime I want to, I just don't want to. I can ask the model instead. I need him to do actual work [AMB-6]: edit, submit pull requests, or open issues. One of those three things. Now I will create the repository, and I need you to take this transcript and put together a lesson plan, as a document.

CY: I will do that. That sounds really good.

VF: Do you have any questions?

### 3. The Three Planned Products

CY: One thing occurred to me as you explained this. I could see people using it for more than companies, for example to interview their relatives.

VF: Oh, don't give it away. There are three products, and I will tell you. The commercial product is that we go on site. We set up either a Docker container or the cloud service, we customize their installation, they do the knowledge capture, and they get the documents. That is number one. We get a service fee and a license fee, plus ongoing support and maintenance. Number two is what you said.

VF: Number two is an autobiographical tool. Either the subject is alive and gets interviewed, or the subject has died and the people who knew them talk about them, or the subject is alive but not available and other people tell the story. In every case we capture knowledge about a person. If the person is alive we can capture how they think: maybe they were in the war, maybe in the motor pool, what they thought about the election, what school was like, what it was like coming to this country. I want to stress that we are not creating a virtual clone. That is weird, and nobody wants it. It is the uncanny valley. What we create is a knowledgeable, trusted confidant, an AI persona who knows the person and who you can talk to.

CY: So years later you can ask what so-and-so would have thought about something.

VF: Yes.

CY: That is so neat. I love it.

VF: That would be a subscription fee, or they can buy it and host it themselves at a fixed cost. There is also commercial value. Imagine if we could have interviewed Steve Jobs about how he made decisions, or someone in marketing about what techniques they use to sell a product. You could interview me about my thoughts on API design and create a virtual avatar. It does not pretend to be the person, but it can answer questions about the person in a socially acceptable way.

VF: The third product is a knowledge marketplace. It is like YouTube, but instead of videos you interact with a learning program. Do you know what adaptive learning is? Do you know what adaptive testing is?

CY: Vaguely. Yes, adaptive testing, I get it.

VF: Adaptive learning is like adaptive testing, except that you are learning instead of being quizzed, though you still get quizzed. I have created an adaptive learning program called the Pedagogue, orchestrated with the model. Expressed as Runvil it becomes much more powerful. It is a teaching tool that builds teachers: it takes the knowledge database and builds the teacher you talk to. It is personalized, and it remembers you.

VF: The third product is a subscription. People sign up and create content, such as knowledge bases. For example, there is a man who knows how to create terrariums and has a beautiful YouTube channel. He could create a learning program that teaches you, offer it, and other people can search for it and interact with it. They pay a fee, perhaps in credits. The people who produce content get a cut of the revenue, as on YouTube. People who consume the content can rate it, and here is the trick: randomly, 10 percent of the people who interact with the content get it for free. If they give a rating, they become eligible for another chance at a free product. So at any given time, 10 percent of users are incentivized to give ratings. That is the three products.

CY: That is really cool. I have another question.

### 4. Idea Protection and Positioning

CY: It is very cool, but at the point where you are talking about a for-profit, isn't that when you have to be careful about telling people the idea too early, so others don't copy it?

VF: I am not worried. There are a million ideas and a shortage of people who can execute on them.

VF: I have shown people my prompts, and my prompts have been open source. I showed them in the Anthropic chat, and people looked at me like I was weird. They did not understand what they were looking at. They said, what is this, there is an image in your prompt, and this looks like garbage. They don't understand what they are seeing.

VF: Cynthia, we are early in the cycle. Nobody understands what is coming. Anthropic's prompts are bad.

CY: Oh, OK.

VF: Let me put it this way. I go to Anthropic's website, where they have their skills, take a skill, put it into Claude, and ask what it thinks. Claude says it promises a lot, but here are some problems. So they don't review the skills they publish using their own model. What does that tell you? They are a for-profit company, and they are not like me.

VF: My motivation and incentives are different. I don't answer to anybody. I have no quarterly earnings and no shareholder call, so we can do what we want. If I say I want this code to be perfect before it ships, I can do that. That is my approach to everything. I never settle for good enough. I polish to perfection. Look at the website, the artwork from Bob, my Boost libraries, my past work, and my prompts, which I keep polishing until they are beautiful. For-profit companies cannot do that, because it is not shareholder value. It is pursuing a personal aesthetic.

CY: Right, they can't.

VF: So I am not worried that anyone will steal this. It is so outlandish. Use AI to interview someone and then create an adaptive learning program. People hear that and think I am a fool. They think I'm free [AMB-8].

CY: I think it is brilliant. It is really cool.

VF: I am working on creating this.

### 5. Class Format and Daily Schedule

CY: When we finish here, I will take these transcripts and put them into a learning plan, with immediate steps and acupuncture [?] [AMB-9].

VF: We need a schedule, and we need signups, so everyone is synchronized on time and I know when is when. More than an hour is too much. We should run it like a classroom.

CY: Yes, let's do an hour.

VF: We should segment the day exactly like school. Fifty minutes of class, then a 10-minute break. Then an hour of individual study, where you go do your own thing. Then maybe lunch. After that, a 50-minute class again. Then a 15-minute break, and then the teacher leaves and the group reconvenes without me.

CY: So you envision the breakout happening later in the day. What about a class, a break, and so on?

VF: This is all flexible. I don't know what works, and we need to feel it out. The idea is a 50-minute class that we transcribe. Then someone takes the transcript, puts it in the repository, and runs the prompts on it. Every time there is a transcript, I want everybody's prompt to run.

VF: I want everyone to make a scribe tool, so we can compare notes. Then they review it, or maybe they have an assignment, and we reconvene and answer questions. One of the ideas is the breakout. All the homework should be in the repository. I have tasked everyone with making a prompt. People will choose their prompts and develop them in the repository so we can see everybody's work. Once a week we will pick two or three people to show the progress of their prompt. I think this would be good.

CY: It is going to be great. Let me be brief, but please listen for one second. I am picturing a morning 50-minute session. Sometime after that comes the breakout where just the team meets. Then after lunch, your time, another 50 minutes. Does that work?

VF: OK. We will try it. If it works, we keep it. If not, we adjust it. We can't know ahead of time.

CY: I know you don't want details, but that is what I am thinking. I think it gives people enough time to come up with more questions for the second teaching session in the afternoon.

VF: Yes, I think it is good.

CY: I will put something together and send it to you first, so you can say yay or nay, and then I will post it in the channel. [AMB-10]

### 6. New Team Members

CY: How do you see the three new people integrating on Monday? They will have missed this first session.

CY: There are three: Carol, what's his name, Justin, which is very exciting, and Hugh Andrew [AMB-11], who I hope will be OK with the nickname Rob and I have given him, which is [cut off].

VF: Wait, who is that?

CY: Andrew is the QA guy, the new QA guy. We want to call him QAndrew, because QA.

VF: His work is different from the code side, but he should understand Runvil too.

CY: It would be good if he understood. If he sat in on this to understand Runvil at first, don't you think?

VF: He can sit in as much as he wants, but his role is QA, so his day-to-day work is separate from the engineering side.

CY: I am excited. I am all prepared.

### 7. Workload, Ownership, and Tacit Knowledge

VF: If I am being honest, I feel I might have bitten off a little more than I can chew. David wasn't wrong when he said I have a big challenge coming.

CY: It is a challenge, but it will sort itself out in a few weeks. Let's get through the classroom part first. Who owns what, so people don't step on each other's toes, will come later.

VF: That is easy. It is already a solved problem. I already have the divisions figured out. The problem is that I put these people on these parts, and then how do they know what to do?

VF: How do they know what to do? The UI is the easiest, because we can copy Visual Studio Code. The gateway is the second easiest, because you can copy Unslav [?].

VF: That part is easy, but the other parts are difficult. That is why we need these classes, so that we understand Runvil. I don't even understand Runvil, because when I make a design choice I forget about it. That is the problem with tacit knowledge. You will read the paper, and it is brilliant.

VF: If you sit down with someone and ask them to tell you everything they know, that never works. If you ask me to tell you everything I know about Runvil, I can't do that, because it is contextual. If you ask how I designed the tool calls, that prompts me, and then I can tell you. I can't tell you anything about any episode of Monk, but if I watch the first three minutes, it all comes back.

CY: My brain works the same way. I get it.

VF: It is how most people think.

CY: Is it? Other people seem to remember why they did something a certain way.

VF: Yes, definitely.

CY: I only have my current information. I forget the things that came before and got me to this point.

VF: But you remember stories.

CY: I do remember stories.

VF: The key insight I had at the beginning of the year is that people remember stories, and they are happy to tell them. That is the paper. You have to read the Sage paper, and it will all be clear.

CY: I was reading it before this call. It is good, and it is very true.

### 8. Repository Setup and Naming

VF: I will create that repository. You don't have to stay, unless you want to, because I will mainly create the repository.

CY: I want to go so I can work on the transcript and get it to you as soon as possible. We meet with everyone in two hours.

VF: That is a long time. You don't have to put this transcript in the repository. [AMB-12]

CY: What is that?

VF: You have to put the transient [?] [AMB-12]. I hope I didn't say anything bad. Did I?

CY: I don't think so.

VF: What do we want to call the repository? Runvil Classroom. Or should I use my last name, Falco Classroom?

CY: Runvil is good. Maybe something like Runvil Sage.

VF: Runvil Classroom it is.

CY: Classroom, OK. Can you put the date in it?

VF: What do you mean, the date of what?

CY: Today's date, so years from now we know which one it is. The title, Runvil Classroom, and the date.

VF: You want the repository name to have a date?

CY: I would find it convenient.

VF: I want to make sure I understand. Let me show you what that looks like. Here is the repository, Runvil Classroom. You want to put a date in the name.

CY: It would be helpful for me, but that is fine.

VF: I heard that, but explain why you want the date here.

CY: A year or six months from now we might restart the classroom in a different way, and the date would tell me which one it is.

VF: We will archive, rename the master branch, and make a new master branch.

CY: All right, it is fine. It is just how my brain works, sorry.

VF: We can also rename the repository later, when we have more than one. That is a good point about multiple repositories. Let's get one working first and split it later.

CY: OK.

VF: I will fix this transcript and put it there, and then you can get the minutes out of it.

CY: That sounds good.

VF: All right, bye. Thank you.

CY: Bye. Thanks.

## Unknowns [?]

- fruit - "I saw it for the fruit" (CY, 0:48, in reply to the question about the Knowledge Capture Corporation).
- pi, z - tools some team members use instead of Cursor (VF, 6:57).
- acupuncture - "immediate steps and acupuncture" (CY, 18:00). See AMB-9.
- Unslav - project to copy for the gateway (VF, 23:32).
- the transient - "You have to put the transient" (VF, 25:59). See AMB-12.

## Ambiguities

- [AMB-1] "I was just putting together some thoughts and suggestions, but they're irrelevant now cause what? How do you even do that?" (CY, 0:14) - The transcript does not say what made the suggestions irrelevant or what "that" refers to. No best guess. Resolution needed: CY to say what she meant.
- [AMB-2] "something to do with Harry, yeah?" and "Sure. Mary." (CY 0:51, VF 0:53) - Two different first names, with no clear connection to the Knowledge Capture Corporation. Resolution needed: confirm who or what is meant.
- [AMB-4] "one of the $200 a month cursor accounts. Or the 20" (VF, 6:36) - The unit and billing period for "the 20" are not stated. Best guess: a $20 per month plan. Resolution needed: confirm the plan.
- [AMB-6] "I need him to do actual work" (VF, 9:44) - "him" has no named referent. Best guess: the AI model, based on the preceding turns. Resolution needed: confirm who is meant.
- [AMB-8] "They think I'm free." (VF, 17:24) - The last word does not fit the sentence and looks mis-transcribed. No best guess. Resolution needed: VF to supply the word.
- [AMB-9] "With immediate steps and acupuncture." (CY, 18:00) - Probably a mis-transcription. No best guess. Resolution needed: CY to supply the intended phrase.
- [AMB-10] "We need a schedule... we need signups" (VF, 18:02) names no owner, and "I'm gonna put something together" (CY, 21:26) does not say whether the document is the lesson plan, the schedule, or both. Resolution needed: confirm the owner of the schedule and signups, and what CY is preparing.
- [AMB-11] "3, Carol. What's his name? Justin... and Hugh Andrew" (CY, 21:53) - The names of the three new team members are unclear: it is not certain that Carol and Justin are two of the three, and "Hugh Andrew" may be a mis-transcription. Resolution needed: confirm the three names.
- [AMB-12] "You don't have to put this transcript in the repo." (VF, 25:52) and "You have to put the transient." (VF, 25:59) - The two statements conflict and the second is garbled. At 28:27 VF says he will fix the transcript and put it in the repository. Best guess: the transcript goes in the repository after VF fixes it. Resolution needed: confirm the decision.
## Corrections Welcome

Please correct anything that was misunderstood or missed.

