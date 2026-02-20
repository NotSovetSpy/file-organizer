# File Organizer (fo)

A utility for organizing and sorting files.

## Installation

### Building from source

1. Build the project:

```bash
cargo build --release
```

1. The executable will be located in `target/release/fo`

### Adding to PATH

#### Linux

Add the following line to your `~/.bashrc` or `~/.zshrc`:

```bash
export PATH="$PATH:<path-to-file-organizer>/target/release"
```

Then reload the configuration:

```bash
source ~/.bashrc  # or ~/.zshrc
```

Or use the absolute path dynamically:

```bash
echo 'export PATH="$PATH:'$(pwd)'/target/release"' >> ~/.bashrc
source ~/.bashrc
```

#### Windows (PowerShell)

Add the path to the executable to the PATH environment variable:

```powershell
$env:Path += ";<path-to-file-organizer>\target\release"
```

For permanent addition use:

```powershell
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";<path-to-file-organizer>\target\release", [EnvironmentVariableTarget]::User)
```

### Cross-compilation for Windows (on Linux)

1. Install the Windows target:

```bash
rustup target add x86_64-pc-windows-gnu
```

1. Build the project:

```bash
cargo build --release --target x86_64-pc-windows-gnu
```

The executable will be in `target/x86_64-pc-windows-gnu/release/fo.exe`

## Usage

After installation, you can use the `fo` command from any directory.
