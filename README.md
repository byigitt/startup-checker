# Startup Manager

A Windows 11 startup manager with an interactive TUI (Terminal User Interface) built in Rust. Manage all your startup applications from one place - including hidden registry entries that don't appear in Task Manager.

## Features

- **Comprehensive Scanning** - Finds startup items from multiple sources:
  - Registry (HKCU\Run, HKLM\Run, RunOnce, WOW6432Node)
  - Startup Folders (User and All Users)
  - Scheduled Tasks (logon/boot triggers)
  - Windows Services (auto-start)

- **Interactive TUI** - Easy-to-use terminal interface with keyboard navigation
- **Enable/Disable** - Toggle startup items on or off
- **Automatic Backups** - Creates JSON backups before making changes
- **Admin Detection** - Shows which items require administrator privileges
- **File Validation** - Highlights items with missing executables

## Installation

### From Source

```bash
git clone https://github.com/byigitt/startup-checker.git
cd startup-checker
cargo build --release
```

The executable will be at `target/release/startup-checker.exe`

### Pre-built Binary

Download the latest release from the [Releases](https://github.com/byigitt/startup-checker/releases) page.

## Usage

```bash
# Run normally (can modify user-level items)
startup-checker.exe

# Run as Administrator (can modify all items)
# Right-click -> Run as administrator
```

## Key Bindings

| Key | Action |
|-----|--------|
| `↑` / `k` | Move up |
| `↓` / `j` | Move down |
| `Space` | Toggle enable/disable |
| `Tab` | Collapse/expand group |
| `a` | Apply pending changes |
| `r` | Refresh list |
| `b` | Create backup |
| `?` | Show help |
| `q` / `Esc` | Quit |

## Color Coding

| Color | Meaning |
|-------|---------|
| **Green** | Enabled items (user-level, can modify) |
| **Yellow [A]** | Requires Administrator to modify |
| **Red** | Executable file is missing |
| **Cyan** | Currently selected item |
| **Gray** | Disabled items |

## Screenshots

```
┌─ Startup Manager v0.1.0   [Admin: No] ─────────────────────┐
│                                                             │
│ ─ Startup Items (123 total) ──────────────────────────────  │
│ ▼ Registry (HKCU\Run) [13 items]                           │
│   [x] Discord                                               │
│   [x] Spotify                                               │
│   [x] Steam                                                 │
│ ▼ Registry (HKLM\Run) [3 items]                            │
│   [x] SecurityHealth [A]                                    │
│   [x] RtkAudUService [A]                                    │
│ ▼ Startup Folder (User) [2 items]                          │
│   [x] MyApp.lnk                                             │
│                                                             │
│ ─ Details ─────────────────────────────────────────────────  │
│ Select an item to see details                               │
└─────────────────────────────────────────────────────────────┘
```

## Startup Sources

### Registry Locations
- `HKCU\Software\Microsoft\Windows\CurrentVersion\Run` - Current user startup
- `HKCU\Software\Microsoft\Windows\CurrentVersion\RunOnce` - Run once for current user
- `HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Run` - All users startup (requires admin)
- `HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\RunOnce` - Run once for all users
- `HKLM\SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Run` - 32-bit apps on 64-bit Windows

### Startup Folders
- `%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup` - Current user
- `%PROGRAMDATA%\Microsoft\Windows\Start Menu\Programs\Startup` - All users (requires admin)

### Scheduled Tasks
- Tasks with logon or boot triggers

### Windows Services
- Services set to start automatically (Auto, Boot, System start types)

## How Disabling Works

| Source | Disable Method |
|--------|----------------|
| Registry | Moves value to `...\Run\AutorunsDisabled` subkey |
| Startup Folder | Renames file with `.disabled` extension |
| Scheduled Tasks | Uses `schtasks /change /disable` |
| Services | Changes start type to Manual (demand start) |

## Backups

Backups are stored in:
```
%LOCALAPPDATA%\startup-checker\backups\
```

Each backup is a JSON file with timestamp: `backup_20240115_143022.json`

## Requirements

- Windows 10/11
- Administrator privileges (for modifying system-level items)

## Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run directly
cargo run --release
```

## License

MIT License - See [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Disclaimer

Use at your own risk. Always create a backup before disabling startup items. Some items are essential for system stability - be careful when disabling security software or system services.
