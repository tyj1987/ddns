# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a cross-platform DDNS (Dynamic DNS) tool built with **Tauri 2.0 + Rust + React** that supports multiple cloud providers (AliCloud, Tencent Cloud/DNSPod, Cloudflare, AWS Route53, and domestic Chinese providers).

**Target Platforms:** Windows, macOS, Linux, Docker containers

**Core Capabilities:**
- Automatic IP detection with multiple fallback methods
- Multi-domain management with per-domain scheduling
- Support for A, AAAA, and CNAME record types
- Encrypted credential storage using platform keychains
- Real-time logging and update history
- Headless mode for Docker/server deployments

---

## Development Commands

### Building the Project

```bash
# Install dependencies (first time only)
npm install

# Build frontend
npm run build

# Development mode (hot reload)
npm run tauri dev

# Production build
npm run tauri build

# Run built application
./src-tauri/target/release/ddns
```

### Rust Development

```bash
# Navigate to Rust source
cd src-tauri

# Run Rust tests
cargo test

# Run specific test
cargo test test_name

# Check code (faster than build)
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy

# Build Rust backend only
cargo build --release
```

### Frontend Development

```bash
# Run frontend dev server (without Tauri)
npm run dev

# Type checking
npx tsc --noEmit

# Lint TypeScript
npm run lint

# Format code
npm run format
```

### Docker Development

```bash
# Build Docker image
docker build -f docker/Dockerfile -t ddns-tool .

# Run with docker-compose
docker-compose -f docker/docker-compose.yml up -d

# View logs
docker-compose -f docker/docker-compose.yml logs -f

# Run in headless mode
docker run -e DDNS_HEADLESS=true -v ./config:/config ddns-tool
```

### Database Management

```bash
# Run migrations
cargo run --bin migrate

# Reset database (deletes all data)
rm -f ~/.config/ddns/data.db
cargo run
```

---

## Architecture Overview

### Technology Stack

- **Frontend:** React 19 + TypeScript + Vite + TailwindCSS + shadcn/ui
- **Backend:** Rust + Tauri 2.0
- **State Management:** Zustand (client) + React Query (server state)
- **Database:** SQLite with SQLx (compile-time checked queries)
- **Async Runtime:** Tokio
- **Security:** tauri-plugin-stronghold for encrypted credential storage

### Project Structure

```
src/                    # React frontend
  ├── components/       # UI components (domains, providers, settings)
  ├── hooks/           # Custom React hooks
  ├── lib/             # API clients, store, utilities
  └── types/           # TypeScript type definitions

src-tauri/             # Rust backend
  ├── src/
  │   ├── commands/    # Tauri IPC command handlers (exposed to frontend)
  │   ├── services/    # Business logic (ip_detector, scheduler, dns_updater)
  │   ├── providers/   # DNS provider implementations
  │   │   ├── provider_trait.rs   # Core abstraction
  │   │   ├── aliyun/             # Aliyun DNS client
  │   │   ├── cloudflare/         # Cloudflare client
  │   │   ├── tencent/            # Tencent/DNSPod client
  │   │   └── aws/                # AWS Route53 client
  │   ├── models/       # Data models (domain, config, credentials)
  │   ├── storage/      # Database and secure credential storage
  │   └── error.rs      # Error types and handling
  └── migrations/       # SQL migrations for SQLite
```

### Communication Flow

**Frontend → Backend (IPC Commands):**
```typescript
// Frontend invokes Tauri command
import { invoke } from '@tauri-apps/api/core';
const domains = await invoke('get_domains');
```

```rust
// Backend implements command
#[tauri::command]
async fn get_domains(pool: State<SqlitePool>) -> Result<Vec<Domain>, String> {
    // Implementation
}
```

**Backend → Frontend (Events):**
```rust
// Backend emits events
app.emit("domain_updated", payload)?;
```

```typescript
// Frontend listens to events
import { listen } from '@tauri-apps/api/event';
await listen('domain_updated', (event) => {
    // Handle update
});
```

### Provider Abstraction

All DNS providers implement the `DNSProvider` trait in [src-tauri/src/providers/provider_trait.rs](src-tauri/src/providers/provider_trait.rs):

```rust
#[async_trait]
pub trait DNSProvider: Send + Sync {
    fn provider_id(&self) -> &str;
    async fn initialize(&mut self, credentials: &Credentials) -> Result<(), ProviderError>;
    async fn list_records(&self, domain: &str) -> Result<Vec<DNSRecord>, ProviderError>;
    async fn update_record(&self, domain: &str, record_id: &str, new_ip: &str) -> Result<UpdateResult, ProviderError>;
    async fn test_connection(&self) -> Result<bool, ProviderError>;
}
```

**Key Design Patterns:**
- **Trait-based abstraction** allows easy addition of new providers
- **Provider registry** pattern for dynamic provider loading
- **Per-provider rate limiting** using governor crate
- **Retry logic** with exponential backoff for transient failures

### IP Detection Service

Located in [src-tauri/src/services/ip_detector.rs](src-tauri/src/services/ip_detector.rs)

Implements multiple detection strategies with automatic fallback:
1. **Third-party APIs** (ipify, ifconfig.me, icanhazip) - Primary method
2. **Network interface parsing** - Direct inspection of local interfaces
3. **DNS-based** (OpenDNS resolver) - Query `myip.opendns.com`
4. **STUN protocol** - For NAT traversal scenarios

The service tries each method in order until one succeeds, with configurable timeout and caching.

### Scheduler Architecture

Located in [src-tauri/src/services/scheduler.rs](src-tauri/src/services/scheduler.rs)

- Uses **Tokio intervals** for per-domain scheduling
- Each domain has its own update interval (30s to 24h)
- **Smart update logic:** Only updates DNS if IP actually changed
- Emits real-time events to frontend on status changes
- Survives application restarts (persistent state in SQLite)

### Data Persistence

**SQLite Database** ([src-tauri/src/storage/database.rs](src-tauri/src/storage/database.rs)):
- Domain configurations
- Update history with timestamps
- Application logs with level filtering
- Uses SQLx for compile-time query validation

**Secure Credential Storage** ([src-tauri/src/storage/secure_store.rs](src-tauri/src/storage/secure_store.rs)):
- API keys and secrets encrypted with AES-256-GCM
- Platform-native keychain integration via tauri-plugin-stronghold
- Credentials never logged or exposed in error messages

### State Management

**Server State** (React Query):
- Domains, logs, provider status
- Automatic caching and refetching
- Optimistic updates for better UX

**Client State** (Zustand):
- UI state (modals, selected items, filters)
- User preferences and settings
- Transient state (loading, errors)

---

## Important Implementation Notes

### Adding a New DNS Provider

1. Implement `DNSProvider` trait in new module under `src-tauri/src/providers/`
2. Add provider to registry in `src-tauri/src/providers/mod.rs`
3. Create corresponding configuration UI component in `src/components/providers/`
4. Add provider option to TypeScript types in `src/types/provider.ts`
5. Update provider selector in frontend to include new option

### Headless Mode (Docker/Server)

Set environment variable `DDNS_HEADLESS=true` to run without GUI:
- Application skips window creation
- Configuration via environment variables or `/config/ddns.yml`
- All logs output to stdout
- Useful for Docker containers and server deployments

### Security Best Practices

- **Never** log API credentials (use `***` masking)
- **Always** validate input on both frontend and backend
- Use **parameterized SQL queries** (SQLx enforces this)
- Store credentials in **platform keychain**, not config files
- Enable **TLS** for all HTTP requests (rustls)

### Error Handling

Backend uses `thiserror` for structured errors:
```rust
#[derive(Debug, thiserror::Error)]
pub enum DNSError {
    #[error("Provider error: {0}")]
    Provider(#[from] ProviderError),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}
```

Frontend should handle errors gracefully with user-friendly messages.

### Database Migrations

1. Create new migration file: `migrations/002_description.sql`
2. SQLx will automatically check migration status at runtime
3. Never modify existing migrations - always create new ones
4. Use `cargo test` to verify migration syntax

---

## Testing

### Unit Tests (Rust)

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test module
cargo test ip_detector
```

### Integration Tests

Located in `src-tauri/tests/`:
```bash
cargo test --test integration_test
```

### E2E Tests

Using Playwright (if configured):
```bash
npm run test:e2e
```

---

## Common Issues

### Tauri Build Fails

```bash
# Install Tauri CLI
cargo install tauri-cli --version "^2.0.0"

# Check system dependencies
# Ubuntu/Debian:
sudo apt install libwebkit2gtk-4.1-dev build-essential

# macOS:
# Install Xcode command line tools
xcode-select --install
```

### Database Locked Error

SQLite database may be locked if multiple instances are running:
```bash
# Kill existing instances
pkill ddns

# Or use a different database file
DDNS_DB_PATH=/tmp/test.db cargo run
```

### Provider API Errors

Check logs for specific error messages:
- **401/403:** Invalid credentials
- **429:** Rate limit exceeded (check provider limits)
- **5xx:** Provider server error (retry logic will handle)

---

## Configuration Files

- **Tauri Config:** [src-tauri/tauri.conf.json](src-tauri/tauri.conf.json) - Window settings, permissions
- **Cargo.toml:** [src-tauri/Cargo.toml](src-tauri/Cargo.toml) - Rust dependencies
- **package.json:** [package.json](package.json) - Node dependencies and scripts
- **Vite Config:** [vite.config.ts](vite.config.ts) - Frontend build configuration
- **TypeScript Config:** [tsconfig.json](tsconfig.json) - TS compiler options

---

## Performance Considerations

- **IP Detection:** Cached for 60 seconds by default to avoid excessive API calls
- **Database:** Uses connection pooling (max 5 connections)
- **Rate Limiting:** Each provider has independent rate limiter
- **Scheduler:** Tokio intervals are precise and don't drift over time
- **Frontend:** React Query caching reduces unnecessary IPC calls

---

## Future Development Roadmap

See [CLAUDE.md plan file](~/.claude/plans/proud-rolling-riddle.md) for detailed implementation phases.

**Next priorities:**
1. Phase 1-2: Project foundation and infrastructure
2. Phase 3-4: IP detection and core providers
3. Phase 5-6: Domain management and scheduling
4. Phase 7-9: Logging, UI polish, Docker support
5. Phase 10-11: Additional providers and testing
