# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

HandFlow is a handpan tablature generator that converts MusicXML files into handpan tablatures. It's a full-stack application with a Rust backend (Actix-web) and Vue.js frontend (Vite).

## Repository Structure

```
HandFlow-GH/
├── backend/          # Main Rust backend (Actix-web + PostgreSQL)
├── backend-demo/     # Demo deployment version
└── frontend/         # Vue 3 + Vite frontend
```

## Development Commands

### Backend (Rust)

```bash
# Run the backend server (from backend/ directory)
cd backend
cargo run

# Build for production
cargo build --release

# Run tests
cargo test

# Check code without building
cargo check

# Format code
cargo fmt

# Lint with clippy
cargo clippy
```

The backend runs on **http://localhost:8080**.

### Frontend (Vue.js)

```bash
# Install dependencies (from frontend/ directory)
cd frontend
npm install

# Run development server
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview
```

The frontend dev server proxies `/api` requests to `http://localhost:8080`.

### Full Stack Development

To develop locally, run both servers:
1. Terminal 1: `cd backend && cargo run`
2. Terminal 2: `cd frontend && npm run dev`

## Architecture

### Backend Architecture

**Framework**: Actix-web 4 with async/await using Tokio

**Database**: PostgreSQL with SQLx for async queries
- Main table: `tabs` (stores uploaded scores with UUID-based IDs)
- Schema fields: id (UUID), file_size, metadata (JSONB), score_data (JSONB), favorite_count, created_at, last_used_at
- Database migrations in `backend/migrations/`

**Key Modules**:
- `src/api/handlers/` - HTTP request handlers
- `src/api/routes.rs` - API route configuration
- `src/parsers/` - MusicXML and MIDI parsing logic
- `src/models/` - Data models and business logic
- `src/db/` - Database operations
- `src/utils/` - Utility functions
- `src/error.rs` - Error handling
- `src/config.rs` - Configuration management
- `src/state.rs` - Application state

**Key Dependencies**:
- `actix-web` - Web framework
- `sqlx` - Async PostgreSQL driver
- `quick-xml`, `roxmltree` - XML parsing for MusicXML
- `midly` - MIDI file parsing
- `handlebars` - Template rendering
- `zip` - Handle .mscz (zipped MusicXML) files
- `serde`, `serde_json` - Serialization
- `tokio` - Async runtime

**API Endpoints** (see backend/README.md):
- `POST /upload` - Upload MusicXML files
- `POST /generate` - Generate tablature
- `GET /recent` - Get recent files
- `GET /favorites` - Get favorited pieces
- `POST /favorites/toggle/:id` - Toggle favorite status

### Frontend Architecture

**Framework**: Vue 3 with Composition API (`<script setup>` syntax)

**Build Tool**: Vite with HMR and optimized bundling

**Directory Structure**:
- `src/components/` - Reusable Vue components
  - `audio/` - Audio playback (Metronome.vue, Player.vue)
  - `controls/` - UI controls (TabControls.vue)
- `src/composables/` - Vue composables for shared logic
  - `useTheme.js` - Theme switching (light/dark mode)
  - `useTabs.js` - Tab library operations
  - `useFavorites.js` - Favorites management
- `src/views/` - Page components
- `src/router/` - Vue Router configuration
- `src/services/` - API service layer
- `src/assets/` - Static assets
  - `styles/` - CSS with theme system
  - `img/` - Images

**Key Dependencies**:
- `vue` - Vue 3 framework
- `vue-router` - Routing
- `axios` - HTTP client
- `@vueuse/core`, `@vueuse/integrations` - Vue composition utilities
- `@phosphor-icons/vue` - Icon library

**State Management**:
- Composables with Vue 3 reactivity (`ref`, `reactive`, `computed`)
- No Vuex/Pinia - state managed locally or via composables

**Theme System**:
- CSS custom properties (CSS variables) for theming
- Light and dark mode support
- Theme colors defined in `src/assets/styles/`
- Primary color: `#185964` (light) / `#60a5fa` (dark)
- Comprehensive color system including difficulty levels and status colors

### Data Flow

1. User uploads `.mscz` file (zipped MusicXML) via frontend
2. Frontend sends file to `POST /upload` endpoint
3. Backend unzips, parses MusicXML, extracts musical data
4. Backend stores score data in PostgreSQL with UUID-based ID
5. Frontend receives parsed data and displays for configuration
6. User selects scale, part, transpose settings
7. Frontend sends configuration to `POST /generate`
8. Backend generates handpan tablature HTML
9. Frontend displays tablature with auto-scroll and MIDI playback

### File Processing Pipeline

**MusicXML Parsing** (`backend/src/parsers/`):
- Unzip `.mscz` files
- Parse XML structure
- Extract notes, rhythms, parts, metadata
- Handle multiple instrument parts

**Handpan Adaptation**:
- Map standard notation to handpan scales (9-13 notes)
- Support for scales: Celtic, D Kurd, etc.
- Auto-transpose to fit handpan range
- Rhythm pattern recognition

## Database Setup

The backend requires a PostgreSQL database. Configuration via environment variables or `.env` file:

```bash
# Required environment variables
DATABASE_URL=postgresql://user:password@localhost/handflow
```

Run migrations:
```bash
cd backend
sqlx database create  # Create database
sqlx migrate run      # Run migrations from migrations/
```

## Important Patterns

### Backend Error Handling

Custom error types in `src/error.rs` with proper HTTP status mapping. Always use `?` operator for error propagation in handlers.

### Frontend Component Communication

- **Props down, events up** for parent-child communication
- **Composables** for shared state across components
- **Provide/inject** for deeply nested component trees (if needed)

### Theme System Usage

Use CSS custom properties for all colors:
```css
color: var(--primary-color);
background: var(--surface-primary);
```

Never hardcode colors. The theme system handles light/dark mode automatically via `useTheme` composable.

### Database Queries

Use SQLx compile-time checked queries with the `query!` macro for type safety:
```rust
sqlx::query!("SELECT * FROM tabs WHERE id = $1", id)
```

### File Upload Handling

Backend uses `actix-multipart` for streaming file uploads. Files are temporarily stored, processed, then cleaned up. The `tempfile` crate manages temporary storage.

## Testing

### Backend Tests
```bash
cd backend
cargo test
```

### Frontend Tests
```bash
cd frontend
npm run test  # If test script exists
```

## Deployment

The project has CleverCloud configuration files in `backend/clevercloud/` and `backend-demo/clevercloud/` for deployment.

## Key Features

- **MusicXML Support**: Parse and process `.mscz` files (zipped MusicXML format)
- **Handpan Scales**: Support for 9-13 note handpans with various scales
- **Auto-Transpose**: Automatically fit notes to handpan range
- **MIDI Playback**: Real-time MIDI synthesis with custom waveform generation
- **Auto-Scroll**: BPM-adjustable auto-scroll during playback
- **Favorites System**: Mark and track favorite pieces
- **Recent Files**: Track recently used files with metadata
- **Responsive UI**: Works on laptops and tablets (optimized for Firefox/Chrome)

## Code Style

### Rust
- Use `cargo fmt` for consistent formatting
- Follow Rust API guidelines
- Prefer `?` operator over explicit error handling
- Use meaningful variable names
- Document public APIs with doc comments (`///`)

### JavaScript/Vue
- Use `<script setup>` syntax for Vue components
- Prefer Composition API over Options API
- Use `const` for variables that don't change
- Follow Vue 3 style guide
- Keep components focused and single-responsibility
