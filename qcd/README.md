# qcd

Quick `cd` — a small Rust CLI that recursively lists directories matching a query, designed to be piped into a fuzzy finder so you can jump into any folder by name.

## Build

```powershell
cargo build --release
```

The binary will be at `target/release/qcd.exe`.

## Install

Move the binary somewhere on your `PATH`:

```powershell
mkdir C:\Users\$env:USERNAME\bin
move .\target\release\qcd.exe C:\Users\$env:USERNAME\bin\
```

## PowerShell setup

Add to your `$PROFILE` (open with `notepad $PROFILE`):

```powershell
$env:Path += ";$env:USERPROFILE\bin"

function f {
    $path = qcd.exe $args | fzf
    if ($path) { Set-Location $path }
}
```

Reload: `. $PROFILE`

Requires [fzf](https://github.com/junegunn/fzf) — install with `winget install fzf`.

## Usage

```powershell
f              # fuzzy-pick any directory under the current one
f src          # pre-filter results containing "src"
```

Arrow keys navigate, typing filters, Enter `cd`s into the selected directory.
