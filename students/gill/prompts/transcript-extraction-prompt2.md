## Purpose

This is an AI tool. It states rules and procedures for an AI agent to follow. It holds no input data.

This tool reads a transcript and gives a concise but complete summary of the primary speaker's points. The output must let readers and agents understand what the speaker said as though still present, years after the speaker's death, and act on the speaker's wishes and instructions.

## Inputs

This tool takes one file holding a conversation transcript. It writes a markdown file.

## Overarching rules

* Run no source control operations.

* Modify no existing file unless the user permits it.

* Keep the cleaned transcript in the repo after the run, because every quotation is checked against it.

## Phases

### Phase 1 - identification of speakers and terminology

Where possible, create a subagent to apply these changes to the transcript (if subagent creation is not possible run this in the controlling thread):

* Correct speaker labels to surname-comma-firstname. Where a label is unknown, determine it from any evidence, and correct references between speakers the same way.

* Correct all terminology for consistent use in labels and narration.

* Leave a speaker's own words as spoken, so quotations stay verbatim. Do not touch names or terms inside them, though do correct likely transcription errors; list the corrected form in the Terms table.

* Remove filler phrases and greetings.

Do not summarise in this phase. Keep every substantive turn, in order, including timestamp.

Write the result to a named file in the repo.

### Phase 2 - identification of the primary speaker

Identify likely candidates and ask the user to confirm. Skip if the user named the primary speaker.

### Phase 3 - extraction of primary speaker judgments

Take the naming-pass file and extract the primary speaker's judgments. A judgment is a rule the speaker would apply again in a situation this transcript does not discuss.

Keep a candidate only if it would change what a reader does in such a situation. Keep every decision, with or without a reason, and every correction, refusal, disagreement and prediction: a correction usually carries its reason. Drop plans, announcements, greetings and logistics. Drop descriptions of how something works, unless a reader needs one to make sense of a judgment. Keep such a description as context, not as an extracted item.

Record a stated reason verbatim. Where a reason is not stated, send the item to the reasons table in Phase 4 and raise an open question for it. Never supply a statement or reason the speaker did not give.

Drop what other participants say, unless it modifies or clarifies the primary speaker. Keep the question whenever it produced a judgment.

Every extracted item carries the speaker, one verbatim fragment, and that fragment's timestamp.

### Phase 4 - final output

Restructure the judgments as a pedagogical tool, so future readers learn what the speaker intended and can read once without reading ahead.

Order the output as an inverted pyramid: a reader with thirty seconds gets the most important material, and one with thirty minutes gets the detail. Put the most important section first. Each section is self-contained, orders its own concepts by dependency, and defines at first use any concept owned by a later section.

Every extracted item carries exactly one verbatim fragment, inline in its sentence, with its timestamp. Do not add a separate evidence line: the inline fragment is the evidence. Record an item whose reason is not stated in the reasons table with an open question, and say nothing about the reason otherwise.

The body must not exceed half of the cleaned transcript's word count. State both counts in the header. This is a maximum not a target - brevity is preferable to unnecessary padding.

Every section carries at least one verbatim fragment, and most paragraphs are one quotation and one sentence. A paragraph with no quotation is a candidate for deletion.

Compress in order: drop candidates that fail the Phase 3 test, cut whole items of lowest signal density, then tighten prose without removing a quotation or timestamp.

Then cut one third of the remaining sentences, choosing those whose removal loses no fact, quotation or timestamp. Repeat until nothing more can be cut without loss.

Where possible, use the speaker's style and phraseology, but not at the expense of clarity.

Record an open question wherever the speaker refers to knowledge without stating it, defers a decision, gives no reason, or leaves an instruction unfinished. For each, give what the speaker said and what is missing, then a question that asks for a story rather than a definition: a specific time, a choice made under pressure, or what went wrong when the rule was broken. Add one follow-up that changes one condition of that story. These are the questions for the next interview.

## Output

The output shall follow this skeleton. Every heading appears, even where the section is empty.

```markdown
# [Surname, Firstname]: [one-line subject of the session]

Primary speaker: [Surname, Firstname]
Source transcript: [path of the input file, exactly as given]
Cleaned transcript: [path of the file written in Phase 1, exactly as written]
Extracted items: [n]. Reasons not stated: [n]. Open questions: [n].
Word counts: cleaned transcript [n], this document [n].

## In one sentence

[One sentence that compresses the whole document.]

## Summary

[Two or three paragraphs. What the speaker was trying to achieve, what he
consistently refused, and what is most at risk of being lost.]

## 1. [Most important theme]

[Prose in the speaker's phraseology. Each extracted item carries one
verbatim fragment with its timestamp, inline.]

## 2. [Next most important theme]

## Principles and instructions

[The items stated as instructions a reader can act on, each in the form
"When [situation], [action], because [reason | not stated]." Where the
numbered sections already carry them in that form, write "None: carried
in sections 1 to n."]

## Reasons not stated

| Item | What was not stated | Question |
| --- | --- | --- |

## Open questions

### Q1. [short name]

- **What he said:** "fragment" [m:ss]
- **What is missing:** [one sentence]
- **Interview question:** [story-eliciting question]
- **Variation:** [one condition changed]

## Terms

| Heard | Used as |
| --- | --- |
```

A worked example of one extracted item, showing the fields in place. It is fictional: none of the words or timestamps below appear in a transcript.

```markdown
### 2.1 Keeping a release note short

Cut a release note to the one change a reader must act on. When the note
grew to eleven bullets, he kept only the migration step and moved the
rest to a list, because "a note nobody finishes is a note nobody acts on"
[0:00].
```

## Before writing

Run every check below. On failure, return to the rule it names.

* Every quoted fragment is verbatim, per Phase 1.

* Every claim carries at least one timestamp, per Phase 3.

* Every extracted item carries one verbatim fragment with its timestamp,
  inline, and no separate evidence line, per Phase 4.

* Every extracted item passes the Phase 3 test.

* The body fits the Phase 4 ceiling, and both counts appear in the header.

* Every section carries a verbatim fragment, and the compression pass has run to exhaustion, per Phase 4.

* Every item whose reason is not stated is a row in the reasons table, and its open question carries a story question and a variation, per Phase 4.

* Every skeleton section appears, in inverted-pyramid order, per Phase 4.
