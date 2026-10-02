# Proposed repository and application folder structure

**Status:** Proposed for implementation planning only. These directories are not product source yet; the only current code-like content is isolated under `spikes/phase-1-windows-proof/` and is explicitly non-shipping.

## Proposed monorepo layout

```text
.
├── .github/
│   ├── workflows/                 # PR checks, pinned Windows build, release-only workflow
│   ├── CODEOWNERS                 # Owner review for security/architecture paths (if configured)
│   └── dependabot.yml             # Only if update policy is owner-approved and offline constraints remain
├── assets/
│   ├── app-icons/                 # Original editable source + generated Windows ICO/PNG
│   ├── fonts/                     # Exact locally bundled font files
│   └── licenses/                  # Font/runtime/third-party license source texts
├── docs/
│   ├── phase-0/                   # Immutable discovery baseline and historical evidence
│   ├── phase-1/                   # ADRs, architecture contracts, ERD, security, delivery/tests
│   ├── phase-2/                   # Design-system specification and review evidence
│   └── phases/                    # Dated phase completion reports, if not stored by phase
├── scripts/
│   ├── build/                     # Reproducible build/release helpers; no secrets
│   ├── audit/                     # Lock/license/SBOM/secret and network scans
│   └── test/                      # Deterministic synthetic fixture/setup helpers
├── spikes/
│   └── phase-1-windows-proof/     # Temporary architecture-only proof; never bundled or imported into app
├── src/                           # React/TypeScript presentation only
│   ├── app/                       # Window shell, route registry and navigation
│   ├── features/                  # Screen/feature composition; no direct SQL/native operations
│   │   ├── setup/
│   │   ├── patients/
│   │   ├── clinical/
│   │   ├── appointments/
│   │   ├── billing/
│   │   ├── inventory/
│   │   ├── staff/
│   │   └── administration/
│   ├── shared/
│   │   ├── ipc/                   # Generated TypeScript types + safe invoke wrapper
│   │   ├── authz/                 # Permission-aware affordances only (not security authority)
│   │   ├── components/            # Approved accessible shared UI primitives
│   │   ├── formatting/            # Display helpers; no authoritative financial math
│   │   └── document-preview/      # Restricted render surface and paper preview
│   └── styles/                    # Tokens, shell styles and app-wide CSS
├── src-tauri/                     # Trusted Rust core and Windows adapters
│   ├── Cargo.toml
│   ├── Cargo.lock                 # Required for reproducible builds
│   ├── build.rs
│   ├── tauri.conf.json
│   ├── capabilities/              # Explicit main/print-window capability grants
│   ├── permissions/               # Narrow generated/custom command permissions
│   ├── migrations/                # Ordered, immutable, checksummed SQL migrations
│   ├── resources/                 # Local templates/fonts/license resources only
│   └── src/
│       ├── main.rs                # Tauri bootstrap and window setup
│       ├── commands/              # Thin typed boundary; derive actor server-side
│       ├── application/           # Use cases, transaction/audit/idempotency coordination
│       ├── domain/                # Business invariants and permission-aware policies
│       ├── infrastructure/
│       │   ├── persistence/       # SQLCipher connection, repositories and migrations
│       │   ├── crypto/            # DPAPI, key lifecycle and attachment encryption
│       │   ├── documents/         # Snapshots, print jobs and template version lookup
│       │   ├── backup/            # Format, stream crypto, journalled restore/scheduler
│       │   ├── files/             # Confined opaque attachment storage
│       │   ├── audit/             # Append-only event/audit-chain writes
│       │   └── windows/           # Minimal DPAPI/dialog/printer/WebView2/task adapters
├── tests/
│   ├── frontend/                  # Component and interaction tests
│   ├── integration/               # Real temporary SQLCipher DB and use-case tests
│   ├── security/                  # Direct service/forged-IPC negative tests
│   ├── e2e/                       # Windows-only real Tauri/WebView2 journeys
│   └── fixtures/synthetic/        # Generated non-identifiable data only
├── .gitignore
├── ARENA.md                       # Durable project memory/phase handoff
├── README.md
├── package.json                   # Pinned Node scripts/dependencies
└── package-lock.json              # Required npm dependency lock
```

`telemetry.rs` is deliberately marked “not expected”: it must not be implemented unless the owner explicitly changes the no-telemetry scope; the preferred final tree omits it entirely.

## Ownership and dependency direction

- `src/` may call only the typed `src-tauri/src/commands` IPC surface. It may not import SQLite, OS, filesystem, shell or crypto libraries.
- `commands/` parses bounded DTOs and resolves the active session; it delegates to `application/` and contains no domain decisions.
- `domain/` depends on stable application abstractions/value types, not React, Tauri UI state, printer drivers or disk paths.
- `infrastructure/` implements persistence, cryptography and OS interfaces behind narrow traits. Windows COM `unsafe` stays under `infrastructure/windows` and receives dedicated regression tests.
- `migrations/` are immutable after release. Any correction is a later migration with a checksum.
- UI assets, templates and fonts are local. External runtime URLs, remote fonts and dev-only assets are not bundled.
- Tests use synthetic fixtures under `tests/fixtures/synthetic/`; never copy real clinic records, supplied activation input, private recovery identities, signing material or production keys into tests/artifacts.

## Build and generated-file rules

- Root Node package scripts call typed frontend lint/type/unit/build. Tauri build starts the local Rust crate with the lockfile and an explicit production profile.
- Generated TypeScript IPC declarations and SBOM/notices have one documented producer and drift check; hand-edited generated outputs are rejected.
- `Cargo.lock`, `package-lock.json`, font/license notices and capability/permission manifests are versioned. `target/`, `node_modules/`, `dist/`, temporary PDF/database data, logs and local secrets are ignored and never committed.
- Architecture spikes remain outside `src/` and `src-tauri/`; after proof, archive their concise method/results under `docs/phase-1/` and do not ship their test binary, static HTML, test key or proof output.

## Review gates before implementation

1. Confirm Tauri command capability generation and exact project identifier naming in a minimal Windows build.
2. Add only modules needed by accepted Phase 1 decisions; do not create empty feature folders or placeholders as production features.
3. Confirm build scripts work from a clean checkout using pinned Node/Rust/Windows toolchain and no developer-local file.
4. Run dependency-tree/license/secret/output inventory against the exact release tree.
