# Exec Command

A Windows GUI application wrapper that executes commands from a text file. This tool is useful when you need to run a program with specific arguments, but can only specify a single executable (like a `.lnk` file replacement).

## Features

- **No Console Window**: Runs as a Windows GUI application (no terminal window flashing)
- **Quote Support**: Properly handles quoted arguments with spaces
- **Transparent Execution**: Passes through stdin/stdout/stderr
- **Exit Code Forwarding**: Returns the child process exit code
- **Argument Appending**: Additional arguments passed to the wrapper are appended to the command

## How It Works

1. The executable reads a `.txt` file with the same name as itself
2. Parses the command from the text file (supporting quoted arguments)
3. Appends any additional arguments passed to the wrapper
4. Executes the command without showing a console window
5. Returns the child process exit code

## Usage

### Setup

1. Compile the application for Windows (see [Building](#building) section)
2. Rename the executable to your desired name (e.g., `myapp.exe`)
3. Create a text file with the same name but `.txt` extension (e.g., `myapp.txt`)
4. Put the command to execute in the text file

### Example

**File: `myapp.txt`**
```
"C:\Program Files\MyApp\program.exe" --flag "argument with spaces"
```

**Running:**
```
myapp.exe
```

This will execute:
```
"C:\Program Files\MyApp\program.exe" --flag "argument with spaces"
```

**With additional arguments:**
```
myapp.exe --extra-arg value
```

This will execute:
```
"C:\Program Files\MyApp\program.exe" --flag "argument with spaces" --extra-arg value
```

### Text File Format

- The text file should contain a single command line
- Newlines (both Unix `\n` and Windows `\r\n`) are automatically replaced with spaces
- Arguments with spaces should be quoted: `"path with spaces"`
- The first token is the program to execute, remaining tokens are arguments

**Examples:**

```
c:\path\to\program.exe arg1 arg2 arg3
```

```
"C:\Program Files\App\program.exe" "C:\path\to\file.txt" --option
```

```
notepad.exe "C:\Users\MyUser\Desktop\file.txt"
```

## Building

### Prerequisites

- Rust toolchain (install from [rustup.rs](https://rustup.rs/))
- Windows target toolchain

### On Windows

```bash
# Build release version
cargo build --release

# The executable will be at: target/release/exec-command.exe
```

### Cross-Compilation from Linux/macOS

```bash
# Install Windows target
rustup target add x86_64-pc-windows-gnu

# Build for Windows
cargo build --release --target x86_64-pc-windows-gnu

# The executable will be at: target/x86_64-pc-windows-gnu/release/exec-command.exe
```

**Note for macOS/Linux users:** You may need to install MinGW-w64:
- **macOS**: `brew install mingw-w64`
- **Linux (Debian/Ubuntu)**: `apt-get install mingw-w64`

### Build Optimization

The release profile is configured for minimal binary size:
- LTO (Link Time Optimization) enabled
- Binary stripping enabled
- Optimized for size (`opt-level = "z"`)

## Error Handling

The application exits silently with the following exit codes:

- **Exit code from child**: If the command executes successfully, returns the child process exit code
- **Exit code 1**: If any error occurs:
  - `.txt` file not found
  - `.txt` file cannot be read
  - Command parsing fails
  - Process spawn fails

## Technical Details

- **Windows Subsystem**: Compiled as a GUI application to prevent console window
- **Argument Parser**: Uses `shell-words` crate for proper quote handling
- **Process Flags**: Uses `CREATE_NO_WINDOW` flag to hide console for child processes
- **Cross-Platform Code**: Includes non-Windows fallback (for development/testing)

## License

See LICENSE file for details.

## Use Cases

- Creating shortcuts to programs with complex arguments
- Deploying application launchers that need to execute specific commands
- Situations where only a single `.exe` can be specified but arguments are needed
- Running GUI applications without a terminal window
