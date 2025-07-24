# Styles Web - Electric Eclipse Theme Showcase

A comprehensive visual style guide and component showcase built with Rust, featuring the Electric Eclipse theme with full light/dark mode support, responsive design, and accessibility features.

## Features

- 🎨 **Complete UI Component Library** - Typography, forms, buttons, tables, cards, and more
- 🌓 **Dynamic Theme Switching** - Instant light/dark mode with system preference detection
- ♿ **WCAG AAA Accessibility** - Full keyboard navigation, screen reader support, high contrast
- 📱 **Fully Responsive** - Mobile-first design that adapts to any screen size
- ⚡ **Progressive Enhancement** - Works without JavaScript, enhanced with HTMX
- 🔒 **Security First** - Comprehensive security headers and CSP configuration
- 🚀 **High Performance** - Server-side rendering, aggressive caching, optimized assets

## Tech Stack

- **Backend**: Rust with Axum web framework
- **Templating**: Maud for type-safe HTML generation
- **Interactivity**: HTMX for progressive enhancement
- **Styling**: Custom CSS with CSS variables for theming
- **Build Tool**: Just for task automation

## Quick Start

### Prerequisites

- Rust (latest stable version)
- Just command runner (`cargo install just`)

### Installation

```bash
# Clone the repository
git clone https://github.com/ArkaitzDev/styles-web.git
cd styles-web

# Install dependencies
cargo build

# Start development server
just dev
```

The server will start at `http://127.0.0.1:3000`

## Development

### Available Commands

```bash
just dev              # Start development server
just dev-verbose      # Start with detailed logging
just build           # Build debug version
just build-release   # Build optimized release version
just stop-dev        # Stop development server
just restart-dev     # Restart development server
just status          # Check server status
just logs            # View server logs
just update-deps     # Update Cargo dependencies
just update-rust     # Update Rust toolchain
just update-all      # Update everything
```

### Project Structure

```
styles-web/
├── src/
│   ├── main.rs          # Server configuration and routing
│   └── views/
│       └── mod.rs       # Maud view templates
├── static/
│   ├── css/electric-eclipse/
│   │   ├── main.css     # Base styles
│   │   ├── light.css    # Light theme
│   │   └── dark.css     # Dark theme
│   ├── js/
│   │   ├── htmx.min.js  # HTMX library
│   │   ├── theme-init.js # Theme initialization
│   │   └── main.js      # Main JavaScript
│   └── data/            # JSON-LD structured data
├── justfile             # Task automation
├── Cargo.toml           # Rust dependencies
└── README.md            # This file
```

## Components Showcase

The application demonstrates a complete set of UI components:

### Typography
- Headings (h1-h6)
- Paragraphs and text blocks
- Lists (ordered, unordered, definition)
- Blockquotes and citations

### Form Elements
- All HTML5 input types
- Textareas and selects
- Form validation states
- Accessible labels and help text

### Interactive Components
- Buttons (multiple variants and sizes)
- Navigation menus
- Accordions (details/summary)
- Dialogs and modals
- Progress indicators

### Data Display
- Tables (basic and responsive)
- Code blocks with syntax highlighting
- Cards and panels
- Badges and alerts

### Layout
- CSS Grid system
- Flexbox utilities
- Responsive spacing
- Container queries ready

## Features in Detail

### Theme System
- Automatic theme detection based on system preferences
- Manual theme toggle with persistence
- Smooth transitions between themes
- Consistent color system with semantic naming

### Accessibility
- Full keyboard navigation support
- Focus visible indicators
- ARIA labels and descriptions
- Skip links for screen readers
- Respects prefers-reduced-motion

### Performance
- Server-side rendering for fast initial load
- Aggressive caching for static assets
- Optimized images and assets
- Minimal JavaScript footprint
- LTO enabled for release builds

### Security
- Comprehensive security headers
- Content Security Policy (CSP)
- X-Frame-Options protection
- Strict referrer policy
- Permissions policy configured

## Deployment

### Building for Production

```bash
# Build optimized release version
just build-release

# Run in production mode
RUST_LOG=info ./target/release/styles-web
```

### Environment Variables

- `RUST_LOG` - Log level (trace, debug, info, warn, error)
- `PORT` - Server port (default: 3000)

### Production Considerations

1. The production server binds to `0.0.0.0:3000` (all interfaces)
2. Ensure proper reverse proxy configuration (nginx, Apache, etc.)
3. Set up SSL/TLS termination at the proxy level
4. Configure appropriate log rotation
5. Monitor server health and performance

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is open source and available under the [MIT License](LICENSE).

## Acknowledgments

- Built with [Axum](https://github.com/tokio-rs/axum) web framework
- HTML templating by [Maud](https://maud.lambda.xyz/)
- Interactivity powered by [HTMX](https://htmx.org/)
- Icons from [Tabler Icons](https://tabler-icons.io/)

---

Made with ❤️ using Rust