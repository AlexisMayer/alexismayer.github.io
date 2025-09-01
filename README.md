# CraftData Website

This project contains the source code for the CraftData website.

## Project Structure

This project uses a feature-oriented directory structure:

  - `index.html`: The main entry point of the website.
  - `common/`: Global styles and scripts.
  - `features/`: Contains subdirectories for each distinct feature (e.g., `services`). Each feature directory co-locates its HTML, CSS, JS, and specific assets.
  - `assets/`: Global assets like images or fonts not tied to a specific feature.

## Getting Started

To view the website, open `index.html` in your web browser.

## Development

- **Styles:** Global styles are in `common/styles/main.css`. Feature-specific styles are in `features/<feature_name>/<feature_name>.css`.
- **Scripts:** Global scripts are in `common/scripts/main.js`. Feature-specific scripts are in `features/<feature_name>/<feature_name>.js`.
- **HTML:** Main pages are in `index.html` and `features/<feature_name>/index.html`.
