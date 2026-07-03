# AGENTS.md

This is the working guide for contributors and coding agents in the
`frame-capture` workspace.

Use it to decide:

1. where documentation belongs,
2. whether a crate or surface is user-facing, public integration, or internal,
3. which related docs, examples, captures, tests, and skills must change together,
4. which validation command should run before handoff.

For Bevy applications, start with `crates/frame-capture-bevy`.

For egui, GPUI, raw wgpu, or host-owned screenshot pipelines, start with
`crates/frame-capture-routes`.

For custom renderer integrations, start with `crates/frame-capture`.

For route catalog tooling, use `crates/frame-capture-mcp`.

## Project Summary

`frame-capture` is a Rust workspace for deterministic screenshots in
wgpu-backed applications.

It provides typed route catalogs, a small environment-variable protocol,
capture session metadata, renderer facades, and read-only MCP tooling for
route catalog inspection.

Its priorities are:

1. **Determinism**: keep route ids, default sizes, frame gates, output paths, and scenarios explicit and reproducible.
2. **Integration fit**: keep renderer-specific screenshot mechanics in the matching facade or host application.
3. **Type safety**: model routes, scenarios, sizes, frames, and output names as validated Rust types.
4. **Tooling**: support read-only route catalog inspection over MCP while capture execution remains host-owned and environment-driven.

## Quick Decision Flow

Before editing, classify the change:

1. **Find the surface in the workspace map.** Use its audience label to decide
   how much public explanation the change needs.
2. **Place documentation by audience and durability.** README files, examples,
   public skills, and any future book are user-facing. Public API contracts and
   implementation boundaries belong in crate or module rustdocs. Repo-local
   maintenance workflow belongs in this guide.
3. **Sync public workflow changes.** If derive syntax, environment variables,
   route metadata, capture sizing, frame gates, output paths, scenarios,
   registered routes, Bevy runtime behavior, MCP tools, or recommended usage
   changes, update docs, examples, tests, captures, affected public skill
   guidance, and this `AGENTS.md` when repo-local agent workflow guidance
   changes.
4. **Validate narrowly.** Run the smallest command that proves the edited
   behavior, generated artifact, or documentation surface is still sound.

## Audience Labels

These labels describe the crate or surface itself, not the documentation file
being edited:

- **User-facing**: normal entry points for application developers.
- **Public integration**: public crates meant for facades, extensions, tooling, or deeper customization. These are usually not the default starting point.
- **Internal**: workspace examples, demos, generated captures, and maintenance surfaces.

## Documentation Placement

### User-Facing Documentation

Treat these surfaces as user-facing:

- the root `README.md`,
- every crate `README.md` in the workspace,
- example source that doubles as executable documentation.

Even README files for public-integration crates should explain:

- who the crate is for,
- what it does,
- what most users should use instead when there is a higher-level facade.

Keep user-facing documentation example-first. Prefer Rust snippets and command
examples over prose-only workflow descriptions.

If a mdBook or `book/` surface is added, treat it as user-facing documentation
and keep it synchronized with the root README, affected crate READMEs, examples,
and public skills.

### Code-Adjacent Documentation

There are no crate-local `docs/ARCHITECTURE.md` files. Do not reintroduce them
for routine implementation rationale. Keep durable implementation notes close to
the code in crate-level or module-level rustdocs, item docs, targeted tests, and
snapshot fixtures.

Keep these topics out of user-facing READMEs unless they are necessary for
normal application usage:

- macro parsing and expansion details,
- subsystem boundaries,
- environment protocol data flow,
- Bevy offscreen capture internals,
- registry and `inventory` wiring,
- MCP server protocol details.

### Skill and Agent Guidance

Use this `AGENTS.md` for repo-local agent and contributor operating rules:
crate navigation, sync requirements, validation expectations, maintainer
workflow notes, and development-only instructions.

`skills/use-frame-capture` and `skills/use-frame-capture-bevy` are public
application-developer guidance, not repo-local maintenance guidance. Keep
maintainer-only details in this guide, rustdocs, tests, or fixtures.

Update affected public skills when public APIs, examples, command workflows,
integration patterns, generated output, or recommended usage change. Update
this `AGENTS.md` when repo-local agent workflow guidance changes.

## Synchronization Rules

When a substantive change modifies a public workflow, derive or attribute
syntax, environment variable contract, route metadata shape, capture sizing
behavior, frame-gating behavior, output-path behavior, scenario
behavior, registered-route API, Bevy capture runtime behavior, or MCP tool
schema:

1. Update the root `README.md` when the change affects integration choice or an end-to-end workflow.
2. Update the affected crate `README.md` files.
3. Update the affected examples under `examples/`.
4. Update crate rustdocs when public API contracts, implementation boundaries,
   or internal behavior notes change.
5. Update tests, including trybuild `.stderr` files for intentional macro diagnostic changes.
6. Update generated example captures only when the intended visual output changes.
7. Update affected public skills and this `AGENTS.md` when repo-local agent
   workflow guidance changes.
8. Keep these surfaces aligned in the same change unless there is a documented reason not to.

Keep `frame-capture.toml` guidance aligned with route macro default-size
behavior.

Keep `justfile` capture recipes aligned with the direct environment-variable
capture commands documented in the root README and examples.

Keep `skills/use-frame-capture` and `skills/use-frame-capture-bevy` aligned with
the current public generic and Bevy integration workflows. Keep this
`AGENTS.md` aligned with repo-local development workflows.

## Workspace Map

### Main User-Facing Entry Points

- `crates/frame-capture-bevy`
  Audience: **User-facing**
  Role: Bevy facade for live mode and deterministic capture mode. Owns the offscreen screenshot runtime, `CaptureReady`, capture window plugin setup, route plugins, and the optional registered-route feature.

- `crates/frame-capture-routes`
  Audience: **User-facing**
  Role: route-only facade for egui, GPUI, raw wgpu, and applications that own their rendering and screenshot pipeline. Reexports shared capture primitives and supports registered function routes.

- `crates/frame-capture`
  Audience: **User-facing**
  Role: target-neutral shared protocol and custom renderer entry point. Defines route specs, capture sessions, environment parsing, typed ids, pixel sizes, output paths, scenarios, frame gates, and route macro reexports.

### Public Integration Crates

- `crates/frame-capture-macros`
  Audience: **Public integration**
  Role: proc-macro crate for route, scenario, and registered-route declarations. Most applications should use the macros reexported by `frame-capture`, `frame-capture-bevy`, or `frame-capture-routes`.

- `crates/frame-capture-routes-bevy`
  Audience: **Public integration**
  Role: Bevy `App` registered-route facade without the screenshot runtime. Used directly when an application wants Bevy-specific route installers but owns capture rendering elsewhere, and by `frame-capture-bevy` with its `registry` feature.

- `crates/frame-capture-mcp`
  Audience: **Public integration**
  Role: read-only MCP server helpers for exposing route catalog metadata over stdio. It lists routes and returns route details; it must not launch captures or save screenshots.

- `crates/frame-capture-toml`
  Audience: **Public integration**
  Role: small TOML parser for package-local capture defaults read by the route macros. Most users configure `frame-capture.toml` instead of depending on this crate directly.

### Examples and Workspace Surfaces

- `examples/bevy`
  Audience: **Internal**
  Role: Bevy screenshot runtime example and source of checked-in example captures under `examples/bevy/captures/`.

- `examples/gpui`
  Audience: **Internal**
  Role: route-only GPUI router example. It is excluded from the root workspace and has its own manifest.

- `frame-capture.toml`
  Audience: **Internal**
  Role: workspace-level default capture size used by route macros when no enum or route-specific size is supplied.

- `justfile`
  Audience: **Internal**
  Role: common formatting, checking, testing, documentation, coverage, and example capture commands.

- `skills/use-frame-capture`
  Audience: **User-facing**
  Role: public skill source for target-neutral and route-only capture workflows.

- `skills/use-frame-capture-bevy`
  Audience: **User-facing**
  Role: public skill source for Bevy screenshot runtime workflows.

## Validation and Editing Rules

### Validation After Changes

- Validation is the default after code or workflow changes.
- Run the narrowest command that proves the edited behavior works for the
  affected crate, docs, examples, captures, MCP tools, or capture workflow.
- Prefer targeted crate, macro, route, MCP, example, or capture recipe checks before full-workspace validation.
- Use `just check`, `just test`, or a more specific `justfile` recipe when the change spans multiple surfaces.
- If validation cannot be run, state why and what remains unvalidated.
- Do not claim a change works unless it was validated, generated from a source of truth, or the remaining risk is explicitly documented.

### When Editing Docs

- Keep READMEs user-facing and example-first.
- Put parsing internals, runtime design, registry mechanics, and protocol
  rationale in rustdocs, tests, or fixtures when that depth is needed.
- Keep the root `README.md`, affected crate READMEs, rustdocs, examples, public
  skills, any future book, and `justfile` commands synchronized.
- Keep affected public skill guidance and this `AGENTS.md` synchronized with
  public workflow and API changes.
- For MCP changes, keep tool names and response shape documentation aligned with tests.

### When Editing Rust Crates

- Use `cargo` or `just` for build, test, and run tasks.
- Keep dependency versions in the workspace root `Cargo.toml`.
- Use `workspace = true` in member crates when depending on workspace-managed dependencies.
- Let each crate choose its own dependency features in its own `Cargo.toml`.
- Keep facade reexports and `prelude` modules aligned with newly public types.
- Prefer validated newtypes for public capture values instead of raw strings or integers.
- Preserve the workspace lint policy in the root `Cargo.toml`.

Useful commands:

- `just fmt`
- `just check`
- `just clippy`
- `just test`
- `just ci`

### When Editing Route Macros or Typed IDs

- Keep derive macros and attribute macros aligned: `CaptureRoute`, `CaptureScenario`, `routes`, and `capture_routes`.
- Keep facade-specific registered-route attributes aligned across `frame-capture-routes`, `frame-capture-routes-bevy`, and `frame-capture-bevy`.
- Keep `frame-capture-toml` parsing behavior aligned with macro default-size lookup.
- Route ids may be relative route paths; scenario ids are state ids, not paths.
- Update trybuild tests and `.stderr` files when diagnostics intentionally change.

### When Editing Registered Routes

- Preserve duplicate route-id validation.
- Keep generated route key names, explicit `key = ...` behavior, and installer signatures documented and tested.
- Keep generic registered routes in `frame-capture-routes` separate from Bevy `App` installers in `frame-capture-routes-bevy`.
- When `frame-capture-bevy` reexports registry APIs, keep the `registry` feature gate accurate in code, tests, and docs.

### When Editing the Bevy Capture Runtime

- Live mode should keep normal Bevy plugin behavior.
- Capture mode should disable the primary window path, use the schedule runner, render to an offscreen image, save a PNG, and exit successfully.
- Respect requested route, size, frame, output path, scenario, and `CaptureReady`.
- Update the Bevy example and checked-in captures only for intentional visual or runtime behavior changes.

### When Editing MCP Route Catalog Tooling

- Keep MCP helpers read-only.
- Do not add capture launching, screenshot saving, or file mutation to the MCP server crate.
- Keep enum route tools and registered-route tools aligned: `list_capture_routes`, `get_capture_route`, `list_registered_capture_routes`, and `get_registered_capture_route`.
- Keep serialized route metadata stable unless the schema change is deliberate and documented.

### When Editing Examples or Captures

- Treat examples as executable documentation.
- Keep route ids, titles, and sizes aligned with README snippets when examples are used to demonstrate a public workflow.
- Prefer adding a focused example route over expanding a demo into unrelated UI complexity.
- Regenerate checked-in PNG captures only when the expected output changes.
- Use direct `FRAME_CAPTURE_*` recipes from the `justfile` and README when regenerating example captures.

### When Writing Tests

- Use regular Rust tests for capture environment parsing, output path resolution, registered route behavior, and MCP route catalog behavior.
- Use trybuild tests for proc-macro diagnostics.
- Prefer [insta](https://insta.rs/) snapshots when comparing structured output or generated code is clearer than assertion-heavy tests.
- Prefer raw multiline strings, or `quote! { ... }` in macro contexts, over escaped single-line literals for embedded Rust code.
