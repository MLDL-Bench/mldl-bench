# Getting Started

Welcome to the **ML Bench Leaderboard** project!  
This guide will help you set up the documentation site locally, preview changes, and contribute effectively.

## Prerequisites
- [Python 3.13+](https://www.python.org/downloads/)
- [pip](https://pip.pypa.io/en/stable/)
- Git installed and configured

## 1. Clone the repository
```bash
git clone https://github.com/MLDL-Bench/mldl-bench.git
cd mldl-bench
```

## 2. Create a Python virtual environment
**Windows**
```bash
py -3 -m venv .mldl-venv
.mldl-venv\Scripts\activate
```

**macOS / Linux**
```bash
python3 -m venv .mldl-venv
source .mldl-venv/bin/activate
```

## 3. Install MkDocs and Material theme
```bash
pip install mkdocs-material
```

## 4. Serve the documentation locally
```bash
mkdocs serve
```
This will start a local development server at:
```
http://127.0.0.1:8000
```
Any changes you make in the `docs/` folder will auto-refresh in your browser.

## 5. Build the site for production
```bash
mkdocs build --strict
```
The static HTML output will be generated in the `site/` folder.

## 6. Deploy with GitHub Pages
Our repo is set up with a GitHub Actions workflow that automatically deploys the docs when changes are pushed to `main`.  
Just commit and push updates to `docs/` or `mkdocs.yml`.

---

### Contributing
- Follow the guidelines in [CONTRIBUTING.md](CONTRIBUTING.md)
- Open a Pull Request for all changes (docs are reviewed like code)
- Keep each doc page focused on a single topic
- Use Mermaid diagrams in Markdown for architecture and data flow

---

### Resources
- [MkDocs Documentation](https://www.mkdocs.org/)
- [Material for MkDocs](https://squidfunk.github.io/mkdocs-material/)
