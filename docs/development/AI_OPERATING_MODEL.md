# AI Development Operating Model

## Responsibilities

User: Product Owner / Traffic-Engineering Domain Authority / final UAT.

ChatGPT: research, product definition, UX/system architecture, standards/source work, task decomposition, acceptance criteria, Codex task contracts, GitHub review, and synthesis.

Codex: repository-grounded implementation, tests, local tooling, executable validation, and bounded fixes.

Independent reviewer/subagent: fresh-context scrutiny for architecture, geometry, UX, licensing, and release qualification when useful.

## Principle

Use the cheapest model that can reliably finish a bounded task. ChatGPT should remove ambiguity before Codex execution so implementation can often run on a lower-cost model without losing reliability.

## Change lifecycle

Research → Specify → Plan → Scrutinize → Implement → Verify → Independent Review → Human UAT when required → Merge → Qualification.

## Task contract

Every important Codex task should identify:
- mission;
- authoritative sources;
- verified baseline;
- goal;
- non-goals;
- architecture constraints;
- required behavior;
- acceptance criteria;
- required tests;
- forbidden shortcuts;
- deliverables;
- stop conditions.

## Agent policy

Prefer one writer per worktree/task. Use parallel read-only reviewers/researchers when the work is separable. Avoid parallel write-heavy agents on the same code surface.
