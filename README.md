*Built with Antigravity and Gemini. This project is a demonstration of AI-assisted engineering.*

# 12 Coins Puzzle

A web-based AI visualization of the classic 12 coins weighing puzzle, built with Rust and Dioxus. This is not my code but rather AI's code, directed by me. It was built with Antigravity and Gemini.

## The Game

This application is an interactive puzzle game where your goal is to identify counterfeit coins among a set using a balance scale. The counterfeit coins may be heavier or lighter than the normal ones. You must use logic and a limited number of weighings to deduce which coins are "bad".

## Features

-   **Interactive Balance Scale**: Drag and drop coins to weigh them against each other with realistic animations.
-   **Configurable Challenges**: Customize the number of total coins and the number of hidden bad coins.
-   **Precision Control**: Set min/max weight deviation factors to make the puzzle easier or harder.
-   **History Tracking**: A detailed log of all your previous weighings and their outcomes.
-   **Advanced Solver**: Stuck? Switch to Solver mode to see a complete mathematical decision tree for the optimal strategy.
-   **Comprehensive Results**: A detailed report at the end showing exactly which coins were fake and their specific weights.

## Built With
- **Rust**: Core logic.
- **Dioxus**: UI framework.
- **Tailwind CSS**: Styling.

## Development

Local:
```bash
npm install
trunk serve
```

### Tailwind CSS Integration
Tailwind is integrated directly into the **Trunk** build pipeline via `Trunk.toml`. 
- **Pre-build Hook**: A hook runs `@tailwindcss/cli` to scan the Rust source files and generate `tailwind.css` before the main build starts.
- **Automatic Updates**: When running `trunk serve`, any changes to your Rust components will trigger a styles rebuild automatically.

## Production Build

To build for release:
```bash
npm install
trunk build --release
```

### Deployment & Optimization

For production, it is a good idea to use **Brotli** to serve the site.

Run this to compress all static artifacts:
```bash
find dist/ -type f -exec brotli --best --force {} \;
```
Configure your server (e.g., Nginx) to serve these pre-compressed files using `brotli_static on;`.