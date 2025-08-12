# PRD: Set up project structure and core interfaces

- **Source**: [Issue #1: Set up project structure and core interfaces](https://github.com/MLDL-Bench/mldl-bench/issues/1)
- **Priority**: High
- **Type**: Epic, Infrastructure
- **Status**: Open
- **Owner**: @davidrmellors
- **Assignee**: @ST10204902
- **Created**: 2025-08-12
- **Version**: 1.0

---

### Overview
Establish the foundational repository structure and core interfaces for the mldl-bench platform across Rust (desktop orchestrator + Tauri), Python (API), and database layers. This work enables rapid iteration on benchmarks, reliable data capture, and a clean path to documentation, CI, and releases.

### Problem Statement
The project requires a coherent, extensible baseline that connects desktop execution, benchmark orchestration, API surfaces, and data storage. Without a well-structured skeleton and clearly defined interfaces, subsequent features risk inconsistency, technical debt, and slow iteration.

### Goals
- **Create a Rust workspace** that houses the Tauri app scaffold and core crates.
- **Define core traits/interfaces** for system info collection, benchmark execution, and result submission.
- **Scaffold a Python FastAPI service** with clear module boundaries and versioned endpoints.
- **Introduce a database migration system** and initial schema to persist device specs, runs, and results.
- **Align docs structure** with MkDocs conventions and ensure local preview works.

### Non-Goals
- Implementing full benchmark suites or advanced scheduling logic.
- Building the leaderboard UI/website.
- Finalizing security hardening and advanced validation pipelines (only baseline hooks here).

### Users and Personas
- **Benchmark Contributor**: Runs benchmarks locally via the desktop app; needs reliable execution and clear logs.
- **Infra Maintainer**: Evolves the API/database schema; needs migrations and typed boundaries.
- **Community Reviewer**: Reads docs/ADRs; needs clear architecture and rationale.

### Key Use Cases
- As a contributor, I can clone the repo, bootstrap environments, and run a basic benchmark flow end-to-end.
- As a maintainer, I can add a new benchmark “suite” with minimal boilerplate and strong typing.
- As a maintainer, I can evolve the schema with migrations and keep dev/prod in sync.

### Functional Requirements
- Rust workspace layout with Tauri app and core crates.
- Traits for:
  - System information collection (e.g., CPU, GPU, RAM, OS, drivers)
  - Benchmark execution lifecycle (prepare → run → collect → package)
  - Result signing/validation hooks and submission to API
- Python FastAPI scaffold with versioned routes (e.g., `/v1/`), Pydantic models for results/specs.
- Database migration tool (e.g., Alembic for Python or SQL migration runner) and seed schema for:
  - Devices, Benchmarks, Runs, Results, Artifacts
- Documentation live preview (`mkdocs serve`) and strict builds (`mkdocs build --strict`).

### Acceptance Criteria
From the issue:
- [ ] Create Rust workspace with Tauri app structure and core crates
- [ ] Define core traits and interfaces for system info, benchmark execution, and result submission
- [ ] Set up Python FastAPI project structure with proper module organization
- [ ] Create database migration system and initial schema files

### Requirements Traceability
- **1.1: Cross-Platform Desktop Application** → Tauri app scaffold + Rust orchestrator
- **1.2: Automated Benchmark Execution** → Execution traits and lifecycle
- **5.1: Data Storage and API** → FastAPI scaffold + DB schema/migrations

See also: `docs/architecture/system-overview.md`, `docs/architecture/data-model.md`, `docs/architecture/validation-and-signing.md`.

### Architecture Notes
- Desktop-first orchestration (Rust) invokes Python benchmark runners within per-suite virtual environments.
- Results and device specs are validated and signed before submission to the API.
- API persists normalized entities and stores artifacts in object storage (future step).
- Versioning: semantic for apps/APIs; calendar for benchmark suites.

### Deliverables
- Repository skeleton with:
  - `tauri/` or `apps/desktop/` scaffold in Rust
  - `crates/` for `mldl-sysinfo`, `mldl-exec`, and shared types
  - `services/api/` FastAPI scaffold with `/v1/`
  - `migrations/` (or `alembic/`) and initial schema SQL
  - Updated `docs/` with Getting Started and architecture references

### Milestones
- **M1: Repo & Docs**
  - Initialize workspaces, add `mkdocs.yml`, verify `mkdocs serve`
- **M2: Rust Core**
  - System info crate (`mldl-sysinfo`) and execution interfaces (`mldl-exec`)
- **M3: API Scaffold**
  - FastAPI `/v1/` routes and Pydantic models for device specs, runs, results
- **M4: Database**
  - Migration tool integrated; initial tables for devices/benchmarks/runs/results/artifacts
- **M5: Wiring**
  - Minimal E2E: desktop → run dummy benchmark → submit result → API persists

### Success Metrics
- Repo builds for Rust crates succeed (`cargo build`) and API starts locally (`uvicorn` or `fastapi dev`).
- `mkdocs build --strict` passes locally.
- A minimal benchmark run produces a persisted result row.

### Risks and Mitigations
- **Over-scoping early crates** → Start with minimal traits, evolve with ADRs.
- **Migration drift** → Enforce migrations in CI and document processes.
- **Cross-platform differences** → CI matrix for macOS/Linux/Windows on critical crates.

### Dependencies
- Rust toolchain, Python 3.13+, Node for Tauri frontend.
- MkDocs Material for documentation.

### Out of Scope (for this PRD)
- Public leaderboard UI and website.
- Advanced trust-tiering and signature verification pipelines.

### Open Questions
- Which migration tool to standardize for the API (Alembic vs. pure SQL runner)?
- Where to host early artifacts (S3-compatible vs. local only)?
- Naming of crates and directory layout (`apps/` vs. `tauri/`, `crates/` vs. `packages/`)?

### References
- Documentation index: `docs/index.md`
- Getting Started: `docs/getting-started.md`
- System overview: `docs/architecture/system-overview.md`
- Data model: `docs/architecture/data-model.md`
- Validation and signing: `docs/architecture/validation-and-signing.md`
- Source issue: [Issue #1](https://github.com/MLDL-Bench/mldl-bench/issues/1)
