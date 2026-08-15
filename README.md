# EyeTrigger

EyeTrigger is a lightweight macOS menu-bar application designed to reduce prolonged screen exposure through adaptive eye-load tracking and staged break reminders.

Instead of relying only on fixed timers, EyeTrigger maintains a continuous **Eye Load** state that increases during active computer use and decreases during breaks, idle periods, and time away from the screen.

The app runs quietly in the background and intervenes only when needed.

---

## Features

- Real-time Eye Load estimation
- Menu-bar Eye Load display
- 20-second micro-break reminders
- Repeated micro-breaks during long work sessions
- 3-minute recovery breaks
- Fullscreen forced recovery when a major break is repeatedly deferred
- Works across macOS Spaces
- Reminders remain visible over fullscreen applications
- No permanent Dock icon during normal use
- Lightweight Rust + Tauri architecture
- Native macOS floating reminder panels

---

## How It Works

EyeTrigger maintains a continuous fatigue state called **Eye Load**.

```text
Active computer use
        ↓
   Eye Load rises
        ↓
 ┌──────┼───────────────┐
 ↓      ↓               ↓
Micro   Recovery      Strong
Break   Break         Recovery
20 s    3 min         5 min
```

The reminder system is adaptive rather than purely timer-based.

Taking breaks reduces Eye Load and naturally delays later reminders.

---

## Eye Load

Eye Load is represented as a normalized value between:

```text
0% ───────────────────────────── 100%

rested                         heavily loaded
```

During active computer use, Eye Load gradually increases according to a fatigue model.

During idle periods and breaks, Eye Load decreases.

This means the reminder schedule adapts naturally to user behavior.

For example:

```text
Eye Load ≈ 50%
      ↓
20-second micro-break
      ↓
Eye Load decreases slightly

      ↓ continue working

another micro-break

      ↓ continue working

Eye Load ≈ 74%
      ↓
3-minute recovery break
```

If the recovery break is completed, Eye Load decreases significantly and a new recovery cycle begins.

If the recovery break is deferred and screen use continues:

```text
Recovery break
     ↓
   Later
     ↓
continued screen use
     ↓
Eye Load ≈ 86%
     ↓
Fullscreen recovery
```

The fullscreen intervention is intentionally uncommon.

Under normal use, completing the recovery break should prevent EyeTrigger from reaching this stage.

---

## Reminder Levels

### Level 1 — Micro Break

A small floating reminder appears when Eye Load reaches a moderate level.

```text
Look away for about 20 seconds

Click here when you're done
```

The user looks away from the screen for approximately 20 seconds and then clicks the reminder.

Completing the micro-break slightly reduces Eye Load.

Multiple micro-break reminders may occur during a long work period.

The reminder is implemented as a native macOS floating panel and can remain visible across Spaces and fullscreen applications.

---

### Level 2 — Recovery Break

When Eye Load becomes high, EyeTrigger requests a more substantial recovery period.

```text
Give your eyes a real break

3:00

[ Start 3 min Break ]   [ Later ]
```

The recommended recovery period is three minutes.

During the break:

- Eye Load gradually decreases
- The screen is left alone
- The user is encouraged to look farther away and move around

If the break is completed successfully, EyeTrigger starts a new recovery cycle.

If the user selects **Later**, the recovery requirement remains pending.

---

### Level 3 — Strong Recovery

If a Level 2 recovery break is deferred and Eye Load continues to rise, EyeTrigger escalates to a strong intervention.

A native macOS fullscreen Space is created with a five-minute recovery countdown.

```text
RECOVERY REQUIRED

Time to step away.

        5:00

No screen.
Look farther away.
Move around and relax.
```

Level 3 is intended to be rare.

It acts as a fallback when a major recovery break has been repeatedly postponed.

---

## macOS Experience

EyeTrigger is designed primarily as a menu-bar utility.

During normal operation, only the current Eye Load is shown in the menu bar.

```text
👁 43%
```

The main application window is hidden by default.

EyeTrigger also stays out of the Dock during normal use.

Clicking the menu-bar indicator opens the status window.

Closing the status window does not stop EyeTrigger.

The background monitoring process continues running.

---

## Main Window

The main window provides a compact overview of the current state.

It displays:

- Current Eye Load
- Activity status
- Current work session duration
- Idle time
- General recovery state

The window is intended mainly for status inspection rather than continuous interaction.

---

## Adaptive Reminder Logic

EyeTrigger does not use a simple fixed timer such as:

```text
Every 20 minutes → show reminder
```

Instead, continuous work time is embedded into the Eye Load dynamics.

A user who continuously works without meaningful rest will see Eye Load rise toward the reminder thresholds.

A user who takes regular breaks will see Eye Load decrease, naturally delaying future reminders.

This allows the system to respond to actual usage behavior rather than only elapsed clock time.

---

## Example Work Cycle

A typical work cycle may look like:

```text
Start working
     ↓
Eye Load rises
     ↓
50%
     ↓
20-second micro-break
     ↓
Load decreases slightly
     ↓
continue working
     ↓
another micro-break
     ↓
continue working
     ↓
74%
     ↓
3-minute recovery break
     ↓
Load decreases significantly
     ↓
new cycle
```

If the recovery break is ignored:

```text
74%
 ↓
Recovery reminder
 ↓
Later
 ↓
continue working
 ↓
86%
 ↓
Fullscreen forced recovery
```

---

## Architecture

EyeTrigger is built using:

- **Rust** for background activity monitoring
- **Tauri 2** for the desktop application layer
- **React**
- **TypeScript**
- **Native macOS NSPanel integration**

The Rust backend is responsible for:

- User activity monitoring
- Eye Load estimation
- Fatigue dynamics
- Reminder trigger logic
- Sleep and suspend handling
- macOS lifecycle handling
- Menu-bar updates
- Native reminder window dispatch

The React frontend is responsible primarily for rendering the user interface.

The reminder logic itself does not depend on the main WebView being visible.

```text
User activity
     ↓
Activity Monitor
     ↓
Fatigue Engine
     ↓
Eye Load
     ↓
Trigger Engine
     ↓
Native Reminder Windows
```

This allows EyeTrigger to continue operating even when the main application window is hidden.

---

## Project Structure

A simplified project structure is:

```text
EyeTrigger/
├── src/
│   ├── App.tsx
│   ├── App.css
│   ├── GentleWindow.tsx
│   ├── GentleWindow.css
│   ├── BreakWindow.tsx
│   ├── BreakWindow.css
│   ├── DimWindow.tsx
│   └── DimWindow.css
│
├── src-tauri/
│   ├── src/
│   │   ├── activity/
│   │   ├── fatigue.rs
│   │   ├── trigger.rs
│   │   ├── session.rs
│   │   ├── lifecycle.rs
│   │   ├── gentle_window.rs
│   │   ├── break_window.rs
│   │   ├── dim_window.rs
│   │   ├── tray.rs
│   │   ├── macos_window.rs
│   │   ├── lib.rs
│   │   └── main.rs
│   │
│   ├── Cargo.toml
│   └── tauri.conf.json
│
├── package.json
└── README.md
```

---

## Installation

Download the latest macOS `.dmg` from the GitHub **Releases** page.

Open the DMG and drag **EyeTrigger** into the Applications folder.

Current builds are primarily intended for Apple Silicon Macs.

---

## macOS Security Notice

Early releases may not yet be notarized with an Apple Developer ID.

If macOS blocks EyeTrigger after installation, open:

```text
System Settings
→ Privacy & Security
```

Then choose:

```text
Open Anyway
```

This limitation can be removed in future releases by adding Developer ID signing and Apple notarization.

---

## Build From Source

### Requirements

You will need:

- macOS
- Node.js
- npm
- Rust
- Cargo
- Tauri build prerequisites

Clone the repository:

```bash
git clone https://github.com/<your-username>/EyeTrigger.git
cd EyeTrigger
```

Install frontend dependencies:

```bash
npm install
```

Run EyeTrigger in development mode:

```bash
npm run tauri dev
```

---

## Build a macOS DMG

Build the release version:

```bash
npm run tauri build -- --bundles dmg
```

The generated DMG can be found under:

```text
src-tauri/target/release/bundle/dmg/
```

For example:

```text
EyeTrigger_0.1.0_aarch64.dmg
```

---

## Development

Check the Rust backend:

```bash
cargo check --manifest-path src-tauri/Cargo.toml
```

Run the application:

```bash
npm run tauri dev
```

Build a release:

```bash
npm run tauri build
```

---

## Current Platform Support

### macOS

Currently supported.

The macOS version includes:

- Menu-bar Eye Load display
- Hidden Dock behavior
- Cross-Space reminders
- Fullscreen application overlays
- Native NSPanel reminders
- Native fullscreen recovery Space
- Sleep and suspend handling

### Windows

Planned.

The Rust fatigue and trigger logic is designed to be reusable across platforms.

Windows support will mainly require platform-specific idle-time detection and window behavior.

### Linux

Planned.

Support may require separate handling for X11 and Wayland environments.

---

## Roadmap

Potential future improvements include:

- User-configurable reminder sensitivity
- Configurable break durations
- Daily Eye Load statistics
- Usage history
- Recovery statistics
- Automatic startup at login
- Improved macOS packaging
- Apple Developer ID signing
- Apple notarization
- Universal macOS binaries
- Windows support
- Linux support
- Optional personalized fatigue models

---

## Release Status

EyeTrigger is currently an early macOS release.

The current development focus is:

- fatigue-model tuning
- reminder behavior
- macOS stability
- native UI integration
- packaging
- distribution

---

## Privacy

EyeTrigger currently relies on local computer activity signals such as idle time.

The core Eye Load calculation runs locally.

No camera is required for the current version.

No cloud service is required for the core fatigue monitoring and reminder functionality.

---

## Philosophy

EyeTrigger is designed around a simple idea:

> Break reminders should respond to accumulated screen load, not just a fixed timer.

Short breaks provide small recovery.

Longer breaks provide substantial recovery.

Strong intervention should only occur when necessary.

The goal is not to interrupt work constantly, but to make prolonged screen use harder to ignore.

---

## Contributing

EyeTrigger is still in an early stage.

Issues, testing feedback, and implementation suggestions are welcome.

If you would like to contribute, please open an issue before making major architectural changes.

---

## License

A license has not yet been selected.

Before making the project widely available for external contributions or redistribution, an appropriate open-source license should be added.
