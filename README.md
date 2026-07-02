# docy
A fast CLI tool to instantly open the relevant language or library documentation in your default browser based on the project files in your current working directory.

---

## Features

- **Automatic Project Detection**: Instantly identifies the project's programming language.
- **Supported Languages**:
  - 🦀 **Rust**: Detects `Cargo.toml` &rarr; Opens [Rust Standard Library Docs](https://doc.rust-lang.org/stable/std/)
  - 🌐 **JavaScript / TypeScript**: Detects `package.json` &rarr; Opens [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/JavaScript)
  - 🐹 **Go**: Detects `go.mod` &rarr; Opens [Go Package Search](https://pkg.go.dev/)
  - 🐍 **Python**: Detects `requirements.txt` &rarr; Opens Python Docs
  - ⚙️ **C++**: Detects `CMakeLists.txt` &rarr; Opens [cppreference](https://cppreference.com/)

---

## Installation

### Install prebuilt binaries via shell script

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/garvittsingla/docy/releases/download/v0.1.3/docy-installer.sh | sh
```

### Install prebuilt binaries via powershell script

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/garvittsingla/docy/releases/download/v0.1.3/docy-installer.ps1 | iex"
```

### Download directly

You can download the precompiled binaries for your platform directly from the [docy 0.1.3 Release Page](https://github.com/garvittsingla/docy/releases/tag/v0.1.3).



---

## Usage

Simply run `docy` inside any supported project folder:

```bash
docy
```

It will scan the directory, identify the language/project type, and open the relevant documentation site in your default browser.

---

