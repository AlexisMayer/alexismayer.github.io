# CraftData Website

This project contains the source code for the CraftData website.

## Project Structure

This project uses a feature-oriented directory structure:

- `public/`: Contains all public-facing files (HTML, CSS, JS, assets).
  - `public/index.html`: The main entry point of the website.
  - `public/common/`: Global styles and scripts.
  - `public/features/`: Contains subdirectories for each distinct feature (e.g., `products`, `services`). Each feature directory co-locates its HTML, CSS, JS, and specific assets.
  - `public/assets/`: Global assets like images or fonts not tied to a specific feature.

## Getting Started

To view the website, open `public/index.html` in your web browser.

## Development

- **Styles:** Global styles are in `public/common/styles/main.css`. Feature-specific styles are in `public/features/<feature_name>/<feature_name>.css`.
- **Scripts:** Global scripts are in `public/common/scripts/main.js`. Feature-specific scripts are in `public/features/<feature_name>/<feature_name>.js`.
- **HTML:** Main pages are in `public/index.html` and `public/features/<feature_name>/index.html`.
