# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust web application showcasing the "Electric Eclipse" theme - a comprehensive visual style guide with full light/dark theme support, responsive design, and accessibility features. Built with Axum, Maud, and HTMX for progressive enhancement.

## Development Commands

The project uses `just` for task automation:

```bash
# Start development server (optimized with 100ms delay)
just dev

# Start development server with verbose logging
just dev-verbose

# Stop development server safely
just stop-dev

# Restart development server
just restart-dev

# Check server status
just status

# View logs in real-time
just logs

# Build project
just build          # Debug build
just build-release  # Release build (optimized)

# Update dependencies
just update-deps    # Update Cargo dependencies
just update-rust    # Update Rust toolchain
just update-all     # Update everything
```

## Architecture

### Tech Stack
- **Framework**: Axum 0.8 (async web framework)
- **Templating**: Maud 0.26 (type-safe HTML in Rust)
- **Interactivity**: HTMX + axum-htmx integration
- **Runtime**: Tokio with all features
- **Middleware**: tower & tower-http for static files, security headers

### Project Structure
```
src/
├── main.rs         # Server setup, routing, middleware configuration
└── views/
    └── mod.rs      # Maud views (render_hello_world, render_not_found)

static/
├── css/electric-eclipse/  # Theme stylesheets
│   ├── main.css          # Base styles
│   ├── light.css         # Light theme variables
│   └── dark.css          # Dark theme variables
├── js/
│   ├── htmx.min.js       # HTMX library
│   ├── theme-init.js     # Theme initialization
│   └── main.js           # Main JavaScript
└── data/                 # JSON-LD structured data
```

### Routes
- `/` - Main style showcase page
- `/api/json-ld/website` - Website structured data
- `/api/json-ld/person` - Person structured data
- `/robots.txt` - Robots file
- `/sitemap.xml` - Sitemap
- `/static/*` - Static assets (cached aggressively)
- `/*` - 404 fallback

### Key Features
1. **Server-side rendering** with Maud for type-safe HTML generation
2. **Progressive enhancement** - works without JavaScript, enhanced with HTMX
3. **Theme system** with instant switching and persistence
4. **Security headers** including CSP, X-Frame-Options, etc.
5. **Accessibility** - WCAG AAA compliance, full keyboard navigation
6. **Performance** - aggressive caching, optimized assets

## Important Implementation Details

### Theme Handling
- Themes are managed via CSS custom properties in light.css/dark.css
- JavaScript (theme-init.js) handles theme switching and persistence
- Server respects theme preference but doesn't require cookies/sessions

### HTMX Integration
- AutoVaryLayer middleware ensures proper HTMX compatibility
- Navigation updates preserve scroll position and theme state
- Progressive enhancement approach - all functionality works without JS

### Security Configuration
The server implements comprehensive security headers:
- Strict CSP policy
- X-Frame-Options: DENY
- X-Content-Type-Options: nosniff
- Strict referrer and permissions policies

### Development vs Production
- Development: Binds to 127.0.0.1:3000 (localhost only)
- Production: Binds to 0.0.0.0:3000 (all interfaces)
- Release builds use full optimization (LTO, strip symbols)

## CSS Architecture

The project uses a modern CSS approach:
- Base styles in main.css
- Theme-specific variables in light.css/dark.css
- Responsive design with mobile-first approach
- Accessibility features like :focus-visible and motion preferences
- Print styles included

Key CSS patterns:
- Hover states only for devices with hover capability
- Focus visible for keyboard navigation
- Consistent spacing using CSS custom properties
- Color system with semantic naming