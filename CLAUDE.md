# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

RBAC admin backend built on axum 0.8 + rbatis 4 (MySQL) + Redis, with a React/antd frontend in `antd-admin/` (Vite, pnpm). The README notes the project is mid-refactor ("正在重构"); `main.rs` still contains experimental routes (`/test`, `/di`, workflow demo) alongside the real API.

## Commands

```bash
cargo run                         # needs MySQL + Redis per config.toml (read from CWD)
cargo test                        # unit tests live inline (#[cfg(test)]) — no tests/ dir content
cargo test no_dangling_refs       # single test: OpenAPI doc check in src/config/api_doc.rs
cargo run --features hotpath      # enable hotpath profiling; report prints on Ctrl+C (graceful shutdown)
just dev                          # backend (cargo watch) + frontend (pnpm dev) together
just build                        # release backend (copies config.toml into target/release) + web build
cd antd-admin && pnpm lint        # frontend lint
```

- DB setup: create `rustdb`, import `rustdb.sql` (per-table DDL also in `docs/sql/`), edit `config.toml` (`[server] addr`, `[db] url`, `[redis] url`).
- Manual API testing: JetBrains HTTP client files in `docs/test/system/*.http` with envs in `docs/test/http-client.env.json`. Log in via `sys_user.http` first (`admin` / `123456`); it stores the token for the rest.
- `.rustfmt.toml` sets `max_width = 200`.
- Logs go to `log/axum-admin.log` (daily rolling via tracing-appender), not stdout.

## Architecture

Request flow per module: `route → handler → service → dao/model`, with `vo` holding request/response DTOs. Each business table (`sys_dept`, `sys_user`, …) has one file in every layer under `system/`, named `sys_<table>_<layer>.rs`.

- **route/** — `build_sys_<x>_route()` returns `Router<Arc<AppState>>`; every endpoint is `POST /system/<x>/<action>` (camelCase, e.g. `queryDeptList`). New route builders must be merged manually in `main.rs` under the `/api` nest.
- **handler/** — thin: extract `State<Arc<AppState>>` + `ValidatedJson<Req>` (`common/extractor.rs`, runs `validator::Validate`), call the service. Each handler carries a `#[utoipa::path]`; it must also be listed in `config/api_doc.rs` — the `no_dangling_refs` test asserts the exact path count (currently 72), so update that number when adding/removing endpoints. Swagger UI at `/swagger-ui`.
- **service/** — business rules; returns `ServiceResult`. Errors are `AppError` (`common/error.rs`, thiserror) which implements `IntoResponse`; `BusinessError` takes a `&'static str`.
- **dao/** — rbatis macros (`crud!`, `impl_select!`, `impl_select_page!`) applied to model structs, plus hand-written SQL via `rb.exec` for anything non-trivial.
- **common/result.rs** — uniform envelope `BaseResponse { code, msg, data }` (code `0` = success); paginated lists return `Paged { total, items }`. Use `ok()`, `ok_result_data()`, etc.

### Auth (`middleware/auth.rs`)
Applied as a `route_layer` on `/api`. Skips `/system/user/login|register|logout`. Otherwise: verify JWT from `Authorization: Bearer`, load session from Redis (`SysUserService::fetch_session_info`), require the Redis-stored token to equal the request token (single active session), then check the request path against the user's permission list (permissions stored with `/api` prefix, stripped before comparison). Admins bypass the check. On success, inserts a `UserSession` extension and a `user_id` header for handlers.

### Cross-cutting
- **AOP**: `#[aspect(Logger)]` / `#[aspect(Timer)]` (`src/aop/aspects/`) come from the git dependency `shakesoft/function-sharp` (crates `aspect-core/macros/std`). Aspects receive args as `dyn Any` and downcast (e.g. `Arc<AppState>`).
- **DI**: two containers live in `AppState` — `shaku` module (`container: Arc<AutoFacModule>`, used via the `Inject`/`InjectProvided` extractors in `src/inject/`) and a `dill` `Catalog`. Both are currently demo-level; real services are static `impl` blocks taking `&RBatis`.
- **Workflow**: `src/workflow/state/` — `state-machines` crate examples (traffic light), not wired into the API.
- **Templates**: sailfish (`templates/*.stpl`, `src/template/`) for the `/` and `/index` pages.
- Middleware stack on `/api`: CORS (any origin), 6s timeout, `TraceLayer` with per-request UUID span, `CatchPanicLayer` returning `AppError::default()`.
