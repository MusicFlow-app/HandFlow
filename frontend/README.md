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

<details>
<summary><strong>🌞 Light Theme</strong></summary>


<table>
<tr>
<th>Variable</th>
<th>Color</th>
<th>Value</th>
<th>Description</th>
</tr>

<!-- Primary Colors -->
<tr><td colspan="4"><strong>Primary Colors</strong></td></tr>
<tr>
<td>--primary-color</td>
<td><div style="width:20px;height:20px;border-radius:50%;background:#185964;border:1px solid #ccc"></div></td>
<td>#185964</td>
<td>Teal blue - Main brand color</td>
</tr>
<tr>
<td>--primary-light</td>
<td><div style="width:20px;height:20px;border-radius:50%;background:#60a5fa;border:1px solid #ccc"></div></td>
<td>#60a5fa</td>
<td>Light blue</td>
</tr>
<tr>
<td>--primary-dark</td>
<td><div style="width:20px;height:20px;border-radius:50%;background:#103E4B;border:1px solid #ccc"></div></td>
<td>#103E4B</td>
<td>Dark teal</td>
</tr>
<tr>
<td>--primary-hover</td>
<td><div style="width:20px;height:20px;border-radius:50%;background:#216B78;border:1px solid #ccc"></div></td>
<td>#216B78</td>
<td>Hover state</td>
</tr>
<tr>
<td>--accent-color</td>
<td><div style="width:20px;height:20px;border-radius:50%;background:#10b981;border:1px solid #ccc"></div></td>
<td>#10b981</td>
<td>Green accent</td>
</tr>

<!-- Background Colors -->
<tr><td colspan="4"><strong>Background Colors</strong></td></tr>
<tr>
<td>--background-color</td>
<td><div style="width:20px;height:20px;border-radius:50%;background:#f4f4f9;border:1px solid #ccc"></div></td>
<td>#f4f4f9</td>
<td>Light gray background</td>
</tr>
<tr>
<td>--container-bg</td>
<td><div style="width:20px;height:20px;border-radius:50%;background:#ffffff;border:1px solid #ccc"></div></td>
<td>#ffffff</td>
<td>White container</td>
</tr>
<tr>
<td>--surface-primary</td>
<td><div style="width:20px;height:20px;border-radius:50%;background:#ffffff;border:1px solid #ccc"></div></td>
<td>#ffffff</td>
<td>Primary surface</td>
</tr>
<tr>
<td>--surface-secondary</td>
<td><div style="width:20px;height:20px;border-radius:50%;background:#f8fafc;border:1px solid #ccc"></div></td>
<td>#f8fafc</td>
<td>Secondary surface</td>
</tr>

<!-- Text Colors -->
<tr><td colspan="4"><strong>Text Colors</strong></td></tr>
<tr>
<td>--text-color</td>
<td><div style="width:20px;height:20px;border-radius:50%;background:#2c3e50;border:1px solid #ccc"></div></td>
<td>#2c3e50</td>
<td>Main text</td>
</tr>
<tr>
<td>--text-light</td>
<td><div style="width:20px;height:20px;border-radius:50%;background:#ffffff;border:1px solid #ccc"></div></td>
<td>#ffffff</td>
<td>Light text</td>
</tr>
<tr>
<td>--text-muted</td>
<td><div style="width:20px;height:20px;border-radius:50%;background:#666666;border:1px solid #ccc"></div></td>
<td>#666666</td>
<td>Muted text</td>
</tr>
</table>

</details>

<details>
<summary><strong>🌙 Dark Theme</strong></summary>


<table>
<tr>
<th>Variable</th>
<th>Color</th>
<th>Value</th>
<th>Description</th>
</tr>

<!-- Primary Colors -->
<tr><td colspan="4"><strong>Primary Colors</strong></td></tr>
<tr>
<td>--primary-color</td>
<td><div style="width:20px;height:20px;border-radius:50%;background:#60a5fa;border:1px solid #ccc"></div></td>
<td>#60a5fa</td>
<td>Bright blue</td>
</tr>
<tr>
<td>--primary-light</td>
<td><div style="width:20px;height:20px;border-radius:50%;background:#93c5fd;border:1px solid #ccc"></div></td>
<td>#93c5fd</td>
<td>Light blue</td>
</tr>
<tr>
<td>--primary-dark</td>
<td><div style="width:20px;height:20px;border-radius:50%;background:#2563eb;border:1px solid #ccc"></div></td>
<td>#2563eb</td>
<td>Dark blue</td>
</tr>
<tr>
<td>--primary-hover</td>
<td><div style="width:20px;height:20px;border-radius:50%;background:#3b82f6;border:1px solid #ccc"></div></td>
<td>#3b82f6</td>
<td>Hover state</td>
</tr>
<tr>
<td>--accent-color</td>
<td><div style="width:20px;height:20px;border-radius:50%;background:#34d399;border:1px solid #ccc"></div></td>
<td>#34d399</td>
<td>Green accent</td>
</tr>

<!-- Background Colors -->
<tr><td colspan="4"><strong>Background Colors</strong></td></tr>
<tr>
<td>--background-color</td>
<td><div style="width:20px;height:20px;border-radius:50%;background:#0f172a;border:1px solid #ccc"></div></td>
<td>#0f172a</td>
<td>Dark blue background</td>
</tr>
<tr>
<td>--container-bg</td>
<td><div style="width:20px;height:20px;border-radius:50%;background:#1e293b;border:1px solid #ccc"></div></td>
<td>#1e293b</td>
<td>Container background</td>
</tr>
<tr>
<td>--surface-primary</td>
<td><div style="width:20px;height:20px;border-radius:50%;background:#1e293b;border:1px solid #ccc"></div></td>
<td>#1e293b</td>
<td>Primary surface</td>
</tr>
<tr>
<td>--surface-secondary</td>
<td><div style="width:20px;height:20px;border-radius:50%;background:#334155;border:1px solid #ccc"></div></td>
<td>#334155</td>
<td>Secondary surface</td>
</tr>

<!-- Text Colors -->
<tr><td colspan="4"><strong>Text Colors</strong></td></tr>
<tr>
<td>--text-color</td>
<td><div style="width:20px;height:20px;border-radius:50%;background:#f1f5f9;border:1px solid #ccc"></div></td>
<td>#f1f5f9</td>
<td>Main text</td>
</tr>
<tr>
<td>--text-light</td>
<td><div style="width:20px;height:20px;border-radius:50%;background:#ffffff;border:1px solid #ccc"></div></td>
<td>#ffffff</td>
<td>Light text</td>
</tr>
<tr>
<td>--text-muted</td>
<td><div style="width:20px;height:20px;border-radius:50%;background:#94a3b8;border:1px solid #ccc"></div></td>
<td>#94a3b8</td>
<td>Muted text</td>
</tr>
</table>

</details>

<details>
<summary><strong>🎯 Status Colors</strong></summary>


<table>
<tr>
<th>Variable</th>
<th>Color</th>
<th>Value</th>
<th>Description</th>
</tr>
<tr>
<td>--status-warning</td>
<td><div style="width:20px;height:20px;border-radius:50%;background:#f97316;border:1px solid #ccc"></div></td>
<td>#f97316</td>
<td>Orange - Warning state</td>
</tr>
<tr>
<td>--status-error</td>
<td><div style="width:20px;height:20px;border-radius:50%;background:#dc2626;border:1px solid #ccc"></div></td>
<td>#dc2626</td>
<td>Red - Error state</td>
</tr>
<tr>
<td>--status-info</td>
<td><div style="width:20px;height:20px;border-radius:50%;background:#2563eb;border:1px solid #ccc"></div></td>
<td>#2563eb</td>
<td>Blue - Info state</td>
</tr>
<tr>
<td>--status-success</td>
<td><div style="width:20px;height:20px;border-radius:50%;background:#059669;border:1px solid #ccc"></div></td>
<td>#059669</td>
<td>Green - Success state</td>
</tr>
</table>

</details>

<details>
<summary><strong>📊 Difficulty Levels</strong></summary>


<table>
<tr>
<th>Variable</th>
<th>Color</th>
<th>Value</th>
<th>Description</th>
</tr>

<!-- Easy -->
<tr><td colspan="4"><strong>Easy</strong></td></tr>
<tr>
<td>--difficulty-easy</td>
<td><div style="width:20px;height:20px;border-radius:50%;background:#4ade80;border:1px solid #ccc"></div></td>
<td>#4ade80</td>
<td>Base color</td>
</tr>
<tr>
<td>--difficulty-easy-gradient-start</td>
<td><div style="width:20px;height:20px;border-radius:50%;background:#22c55e;border:1px solid #ccc"></div></td>
<td>#22c55e</td>
<td>Gradient start</td>
</tr>
<tr>
<td>--difficulty-easy-gradient-end</td>
<td><div style="width:20px;height:20px;border-radius:50%;background:#16a34a;border:1px solid #ccc"></div></td>
<td>#16a34a</td>
<td>Gradient end</td>
</tr>

<!-- Medium -->
<tr><td colspan="4"><strong>Medium</strong></td></tr>
<tr>
<td>--difficulty-medium</td>
<td><div style="width:20px;height:20px;border-radius:50%;background:#fbbf24;border:1px solid #ccc"></div></td>
<td>#fbbf24</td>
<td>Base color</td>
</tr>
<tr>
<td>--difficulty-medium-gradient-start</td>
<td><div style="width:20px;height:20px;border-radius:50%;background:#f59e0b;border:1px solid #ccc"></div></td>
<td>#f59e0b</td>
<td>Gradient start</td>
</tr>
<tr>
<td>--difficulty-medium-gradient-end</td>
<td><div style="width:20px;height:20px;border-radius:50%;background:#d97706;border:1px solid #ccc"></div></td>
<td>#d97706</td>
<td>Gradient end</td>
</tr>

<!-- Hard -->
<tr><td colspan="4"><strong>Hard</strong></td></tr>
<tr>
<td>--difficulty-hard</td>
<td><div style="width:20px;height:20px;border-radius:50%;background:#f87171;border:1px solid #ccc"></div></td>
<td>#f87171</td>
<td>Base color</td>
</tr>
<tr>
<td>--difficulty-hard-gradient-start</td>
<td><div style="width:20px;height:20px;border-radius:50%;background:#ef4444;border:1px solid #ccc"></div></td>
<td>#ef4444</td>
<td>Gradient start</td>
</tr>
<tr>
<td>--difficulty-hard-gradient-end</td>
<td><div style="width:20px;height:20px;border-radius:50%;background:#dc2626;border:1px solid #ccc"></div></td>
<td>#dc2626</td>
<td>Gradient end</td>
</tr>
</table>

</details>

<details>
<summary><strong>⚙️ Other Variables</strong></summary>

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
</details>

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
