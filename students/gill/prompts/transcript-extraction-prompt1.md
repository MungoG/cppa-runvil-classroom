## Purpose

This is an AI tool. It describes rules and prodedures for an AI agent to follow. It does not contain any input data.

The intent of this tool is to read a transcript and, for the primary speaker, provide a concise but full summary of the points made by them. The output should be sufficient that, in the years following the death of the primary speaker, readers and agenets will have sufficient information be able to understand what the speaker said as though they had been present in the meeting, and continue to act on the wishes and instructions of said speaker.

## Inputs

This tool shall be provided with a file containing the transcript of a conversation. It shall write the output in the form of a markdown formatted file.

## Overarching rules

* No source control operations shall be done while running this tool.

* No existing files may be modified in any way unless explicitly permitted by the user.

## Phases

### Phase 1 - identification of speakers and terminology

Where possible create a subagent to read the transcript and produce a version with the following modifications:

* The names of all speakers corrected and in the form surname-comma-firstname. Where a speaker name is unknown determine it from any evidence available. Where one speaker refers to another the output from phase 1 shall modify such reference to use the same format and value.

* All terminology corrected and used consistently.

* All filler phrases and extraneous human interaction such as greetings removed.

The resulting output should be written to a named file in the repo.

### Phase 2 - identification of the primary speaker

Identify likely candidates for the primary speaker and ask the user to confirm who the primarly speaker is. This step may be skipped if the aforementioned primary speaker identity was provided by the person running the tool.

### Phase 3 - extraction of primary speaker statements

Take the output file from the naming pass and extract all points and factual statements made by the primarly speaker. Content said by other participants in the transcript may be eliminated as irrelevant except insofar as it modifies or clarifies the meaning of anything said by the primary speaker, or provides necessary context for anything stated earlier or later by the primary speaker. 

### Phase 4 - final output

Restructure the primary speaker statements so that they are suitable as a pedagogical tool for future readers to learn what the primary speaker's intentions and statements. The output should follow a logically consistent ordering and introduce concepts appropriately so it can be used by readers without the need to read non-sequentially.

Under no circumstances invent statements or reasoning that has not been supplied. If a statement or reasoning is not provided then state as much.

Where possible the output should use the same style and phraseology as the primary speaker, but not at the expense of clarity.

## Before writing

* Every quoted fragment appears verbatim in the cleaned transcript.

* Every claim carries at least one timestamp.

* No sentence asserts a fact or a reason the speaker did not give.

* Every extracted item passes the transfer test.

* The document contains no statement of the form "X is Y" with no judgment attached.

