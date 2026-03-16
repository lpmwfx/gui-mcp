# proj/ -- AI Development Project Control

This directory is an internal project management system designed for AI-assisted development.

It is **not** documentation for end users. It exists so that an AI agent can pick up any session, read these files, and immediately understand what has been done, what to do next, and what to avoid.

## Files

| File | Purpose |
|------|---------|
| `PROJECT` | Single source of truth: goal, stack, structure, current phase, implemented features |
| `TODO` | Task list with IDs, pass criteria, and status -- scoped to the current phase |
| `PHASES` | Roadmap across all phases -- what was done and what is planned |
| `RULES` | Active code rules and project conventions the AI must follow |
| `FIXES` | Known limitations and gotchas -- prevents the AI from repeating past mistakes |
| `ISSUES` | Open issues and bugs under investigation |
| `rulestools.toml` | Scanner configuration for automated rule enforcement |

## Workflow

Every AI session starts the same way:

1. Read `PROJECT` -- understand what this project is and where it stands
2. Read `RULES` -- know the constraints before writing code
3. Read `FIXES` -- don't repeat known problems
4. Read `TODO` -- see what needs to be done

This sequence is enforced by the startup checklist in the rules system. Skipping it leads to wasted work and repeated mistakes.

## Principles

- **One source of truth per concern.** Phase is in PROJECT, not scattered across files.
- **Tasks have pass criteria.** Every TODO item defines what "done" looks like.
- **History is preserved.** Completed phases stay documented in PHASES.
- **Rules are machine-enforced.** Scanners and pre-commit hooks block violations automatically.
