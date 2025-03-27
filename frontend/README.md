# HandFlow Frontend

> Modern Vue.js application for handpan tablature generation and visualization. Built with Vue 3 + Vite for optimal performance and developer experience.

## Table of Contents

- [Overview](#overview)
- [Project Structure](#project-structure)
- [Theme System](#theme-system)
- [Development](#development)
- [Best Practices](#best-practices)

## Overview

HandFlow is a web application that helps musicians convert standard music notation into handpan tablature. It features:

- 🎵 MusicXML file support
- 🎨 Modern, responsive UI
- 🌓 Light/Dark theme
- 🎼 Built-in metronome
- ⭐ Favorites system
- 🔄 Auto-scroll functionality

## Project Structure

```
frontend/
├── public/                 # Static public assets
│   └── audio/              # Audio samples like handpan or metronomes
│   └── favicon/            # Favicon
├── src/                    # Source code
│   ├── assets/             # Project assets
│   │   ├── images/         # Image files
│   │   └── styles/         # CSS modules
│   ├── components/         # Vue components
│   │   ├── audio/          # Audio playback
│   │   │   ├── Metronome.vue
│   │   │   └── Player.vue
│   │   └── controls/       # UI controls
│   │       └── TabControls.vue
│   ├── composables/        # Vue composables
│   │   ├── useTheme.js     # Theme management
│   │   ├── useTabs.js      # Tab librarys operations
│   │   └── useFavorites.js # Favorites system
│   ├── router/             # Vue Router config
│   │   └── index.js
│   └── App.vue            # Root component
├── index.html             # Entry HTML
├── package.json           # Dependencies
└── vite.config.js        # Vite configuration
```


## Theme System

> HandFlow uses a comprehensive theming system with CSS variables for consistent styling across light and dark modes.

### Quick Reference

- **Main Brand Color**: `#185964` (Light) / `#60a5fa` (Dark)
- **Accent Color**: `#10b981` (Light) / `#34d399` (Dark)
- **Background**: `#f4f4f9` (Light) / `#0f172a` (Dark)

### Color Palettes

#### 🌞 Light Theme

##### Primary Colors

| Variable | Color | Value | Description |
|----------|--------|--------|-------------|
| `--primary-color` | ![](https://img.shields.io/badge/-%20-185964?style=flat-square&color=185964) | `#185964` | Teal blue - Main brand color |
| `--primary-light` | ![](https://img.shields.io/badge/-%20-60a5fa?style=flat-square&color=60a5fa) | `#60a5fa` | Light blue |
| `--primary-dark` | ![](https://img.shields.io/badge/-%20-103E4B?style=flat-square&color=103E4B) | `#103E4B` | Dark teal |
| `--primary-hover` | ![](https://img.shields.io/badge/-%20-216B78?style=flat-square&color=216B78) | `#216B78` | Hover state |
| `--accent-color` | ![](https://img.shields.io/badge/-%20-10b981?style=flat-square&color=10b981) | `#10b981` | Green accent |

##### Background Colors

| Variable | Color | Value | Description |
|----------|--------|--------|-------------|
| `--background-color` | ![](https://img.shields.io/badge/-%20-f4f4f9?style=flat-square&color=f4f4f9) | `#f4f4f9` | Light gray background |
| `--container-bg` | ![](https://img.shields.io/badge/-%20-ffffff?style=flat-square&color=ffffff) | `#ffffff` | White container |
| `--surface-primary` | ![](https://img.shields.io/badge/-%20-ffffff?style=flat-square&color=ffffff) | `#ffffff` | Primary surface |
| `--surface-secondary` | ![](https://img.shields.io/badge/-%20-f8fafc?style=flat-square&color=f8fafc) | `#f8fafc` | Secondary surface |

##### Text Colors

| Variable | Color | Value | Description |
|----------|--------|--------|-------------|
| `--text-color` | ![](https://img.shields.io/badge/-%20-2c3e50?style=flat-square&color=2c3e50) | `#2c3e50` | Main text |
| `--text-light` | ![](https://img.shields.io/badge/-%20-ffffff?style=flat-square&color=ffffff) | `#ffffff` | Light text |
| `--text-muted` | ![](https://img.shields.io/badge/-%20-666666?style=flat-square&color=666666) | `#666666` | Muted text |

#### 🌙 Dark Theme

##### Primary Colors

| Variable | Color | Value | Description |
|----------|--------|--------|-------------|
| `--primary-color` | ![](https://img.shields.io/badge/-%20-60a5fa?style=flat-square&color=60a5fa) | `#60a5fa` | Bright blue |
| `--primary-light` | ![](https://img.shields.io/badge/-%20-93c5fd?style=flat-square&color=93c5fd) | `#93c5fd` | Light blue |
| `--primary-dark` | ![](https://img.shields.io/badge/-%20-2563eb?style=flat-square&color=2563eb) | `#2563eb` | Dark blue |
| `--primary-hover` | ![](https://img.shields.io/badge/-%20-3b82f6?style=flat-square&color=3b82f6) | `#3b82f6` | Hover state |
| `--accent-color` | ![](https://img.shields.io/badge/-%20-34d399?style=flat-square&color=34d399) | `#34d399` | Green accent |

##### Background Colors

| Variable | Color | Value | Description |
|----------|--------|--------|-------------|
| `--background-color` | ![](https://img.shields.io/badge/-%20-0f172a?style=flat-square&color=0f172a) | `#0f172a` | Dark blue background |
| `--container-bg` | ![](https://img.shields.io/badge/-%20-1e293b?style=flat-square&color=1e293b) | `#1e293b` | Container background |
| `--surface-primary` | ![](https://img.shields.io/badge/-%20-1e293b?style=flat-square&color=1e293b) | `#1e293b` | Primary surface |
| `--surface-secondary` | ![](https://img.shields.io/badge/-%20-334155?style=flat-square&color=334155) | `#334155` | Secondary surface |

##### Text Colors

| Variable | Color | Value | Description |
|----------|--------|--------|-------------|
| `--text-color` | ![](https://img.shields.io/badge/-%20-f1f5f9?style=flat-square&color=f1f5f9) | `#f1f5f9` | Main text |
| `--text-light` | ![](https://img.shields.io/badge/-%20-ffffff?style=flat-square&color=ffffff) | `#ffffff` | Light text |
| `--text-muted` | ![](https://img.shields.io/badge/-%20-94a3b8?style=flat-square&color=94a3b8) | `#94a3b8` | Muted text |

#### 🎯 Status Colors

| Variable | Color | Value | Description |
|----------|--------|--------|-------------|
| `--status-warning` | ![](https://img.shields.io/badge/-%20-f97316?style=flat-square&color=f97316) | `#f97316` | Orange - Warning state |
| `--status-error` | ![](https://img.shields.io/badge/-%20-dc2626?style=flat-square&color=dc2626) | `#dc2626` | Red - Error state |
| `--status-info` | ![](https://img.shields.io/badge/-%20-2563eb?style=flat-square&color=2563eb) | `#2563eb` | Blue - Info state |
| `--status-success` | ![](https://img.shields.io/badge/-%20-059669?style=flat-square&color=059669) | `#059669` | Green - Success state |

#### 📊 Difficulty Levels

#### Easy

| Variable | Color | Value | Description |
|----------|--------|--------|-------------|
| `--difficulty-easy` | ![](https://img.shields.io/badge/-%20-4ade80?style=flat-square&color=4ade80) | `#4ade80` | Base color |
| `--difficulty-easy-gradient-start` | ![](https://img.shields.io/badge/-%20-22c55e?style=flat-square&color=22c55e) | `#22c55e` | Gradient start |
| `--difficulty-easy-gradient-end` | ![](https://img.shields.io/badge/-%20-16a34a?style=flat-square&color=16a34a) | `#16a34a` | Gradient end |

#### Medium

| Variable | Color | Value | Description |
|----------|--------|--------|-------------|
| `--difficulty-medium` | ![](https://img.shields.io/badge/-%20-fbbf24?style=flat-square&color=fbbf24) | `#fbbf24` | Base color |
| `--difficulty-medium-gradient-start` | ![](https://img.shields.io/badge/-%20-f59e0b?style=flat-square&color=f59e0b) | `#f59e0b` | Gradient start |
| `--difficulty-medium-gradient-end` | ![](https://img.shields.io/badge/-%20-d97706?style=flat-square&color=d97706) | `#d97706` | Gradient end |

#### Hard

| Variable | Color | Value | Description |
|----------|--------|--------|-------------|
| `--difficulty-hard` | ![](https://img.shields.io/badge/-%20-f87171?style=flat-square&color=f87171) | `#f87171` | Base color |
| `--difficulty-hard-gradient-start` | ![](https://img.shields.io/badge/-%20-ef4444?style=flat-square&color=ef4444) | `#ef4444` | Gradient start |
| `--difficulty-hard-gradient-end` | ![](https://img.shields.io/badge/-%20-dc2626?style=flat-square&color=dc2626) | `#dc2626` | Gradient end |

#### ⚙️ Other Variables

```css
/* Spacing */
--spacing-xs: 0.25rem
--spacing-sm: 0.5rem
--spacing-md: 1rem
--spacing-lg: 1.5rem
--spacing-xl: 2rem

/* Border Radius */
--border-radius-sm: 0.25rem
--border-radius-md: 0.375rem
--border-radius-lg: 0.5rem
--border-radius-xl: 1rem

/* Typography */
--font-primary: Arial, sans-serif
--font-display: 'Dancing Script', cursive
--font-size-sm: 0.875rem
--font-size-md: 1rem
--font-size-lg: 1.25rem
--font-size-xl: 1.5rem
```

## Architecture

### Component Architecture
- Modular component design
- Props and events for component communication
- Composition API for state management
- Reusable composables for shared logic

### State Management
- Vue 3 Reactivity system
- Composables for feature-specific state
- Local component state with `ref` and `reactive`
- Computed properties for derived state

### Theme System
- Light and dark mode support
- CSS variables for consistent theming
- Dynamic theme switching
- Responsive design utilities

## Key Features

### File Management
- Drag & drop file upload
- Recent files tracking
- Favorites system
- File metadata display

### Audio Controls
- Built-in metronome
- BPM synchronization
- Audio playback controls
- Visual tempo indicators

### Tablature Display
- Responsive tablature rendering
- Auto-scroll functionality
- Scale visualization
- Note highlighting

### User Interface
- Modern, clean design
- Responsive layout
- Smooth transitions
- Intuitive controls

## Technical Stack

- **Framework**: Vue 3 with Composition API
- **Build Tool**: Vite
- **State Management**: Vue refs and reactive
- **Styling**: CSS modules with variables
- **Animations**: CSS transitions and Vue transitions

## Development

```bash
# Install dependencies
npm install

# Start development server
npm run dev

# Build for production
npm run build

# Run tests
npm run test

# Lint code
npm run lint
```

## Development Workflow

### Setup
1. Clone the repository
2. Install dependencies with `npm install`
3. Create a `.env` file based on `.env.example`
4. Start the development server with `npm run dev`

### Code Organization
- Follow the established project structure
- Keep components focused and single-responsibility
- Use composables for shared logic
- Maintain consistent naming conventions

### Testing
- Write unit tests for composables
- Test components with Vue Test Utils
- Run tests before committing
- Maintain good test coverage

### Style Guide
- Follow Vue 3 style guide recommendations
- Use TypeScript for type safety
- Document complex functions and components
- Use ESLint and Prettier for code formatting

## Best Practices

### Component Design
- Use Vue 3 `<script setup>` syntax
- Keep components small and focused
- Use props and events for communication
- Document component APIs

### State Management
- Use composables for complex state
- Leverage Vue 3 reactivity system
- Keep state close to where it's used
- Use computed properties for derived state

### Performance
- Lazy load routes and components
- Use v-show for frequent toggles
- Optimize computed properties
- Use proper key attributes in v-for

### Accessibility
- Use semantic HTML elements
- Provide ARIA labels where needed
- Ensure keyboard navigation
- Test with screen readers
