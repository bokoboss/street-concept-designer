# Windows Distribution & Office-PC Policy

Verified research date: 2026-08-29

## Product requirement

Street Concept Designer is a **Windows-first desktop application** and must support professional environments where the user may not have Administrator privileges or permission to perform a conventional machine-wide installation.

The product therefore targets two first-class delivery modes:

1. **Installed** — normal per-user Windows installer.
2. **Portable** — unzip/copy and run without installation.

An offline-capable variant is required for constrained environments, but its exact WebView2 packaging strategy remains evidence-gated until release engineering proves it.

This policy is about **runtime distribution**. Development machines may still require Rust/Node/C++ build tooling; end-user machines must not.

---

# Distribution profiles

## D1 — Standard Per-User Installer

Status: REQUIRED.

Target artifact:
- NSIS setup executable preferred initially;
- MSI may be provided later if enterprise deployment requires it.

Requirements:
- installs for current user by default;
- no Administrator privilege required;
- installs under a user-writable application location such as the Tauri per-user default;
- creates normal shortcuts/file association only when intentionally configured;
- checks/provisions WebView2 according to release configuration;
- core application works offline after installation;
- uninstall cleanly removes application files without deleting user project files.

Tauri 2 current documentation states that the default Windows installer mode installs for the current user and does not require Administrator privileges; per-machine installation is a separate explicit option.

Reference:
https://v2.tauri.app/distribute/windows-installer/

Preferred WebView2 strategy for normal online installer:
- Evergreen Runtime;
- installer handles missing-runtime edge cases;
- exact bootstrapper/offline installer choice is release-configuration specific.

---

## D2 — Offline Per-User Installer

Status: REQUIRED FOR RELEASE QUALIFICATION / enterprise-friendly distribution.

Purpose:
allow installation when office/site computers cannot access the internet during setup.

Candidate Tauri configuration:
- WebView2 `offlineInstaller`.

Current Tauri documentation describes `offlineInstaller` as not requiring internet and adding the standalone WebView2 installer to the bundle.

This distribution remains an installer, but:
- should retain per-user/no-admin installation where technically supported;
- should not require Microsoft CDN access during setup.

Reference:
https://v2.tauri.app/distribute/windows-installer/

---

## D3 — Portable Light

Status: REQUIRED PRODUCT CAPABILITY; PACKAGING METHOD EVIDENCE-GATED.

Target artifact:
`StreetConceptDesigner_<version>_Portable.zip`

User flow:
1. copy/download ZIP;
2. extract to a writable local folder;
3. run `StreetConceptDesigner.exe`;
4. no installer;
5. no Administrator privileges;
6. no Node.js, Python, Rust, database server, local web server, or Windows service.

Runtime assumption:
- compatible Evergreen WebView2 Runtime is already present on the computer.

Tauri supports building the application without the normal bundling step using `tauri build --no-bundle`. The project may use that output plus required resources to create and qualify its own portable ZIP artifact.

Important:
Tauri's standard Windows distribution documentation centers on MSI/NSIS installers. Therefore **Portable ZIP is a project-defined release artifact**, not something we should assume is automatically supported merely because a bare executable can be produced.

References:
- https://v2.tauri.app/distribute/
- https://v2.tauri.app/distribute/windows-installer/

Portable acceptance must be proven on clean office-PC test environments before release.

---

## D4 — Portable Offline

Status: REQUIRED OUTCOME / IMPLEMENTATION EVIDENCE-GATED.

Purpose:
support computers where:
- installation is not allowed;
- no internet is available;
- the required Evergreen WebView2 Runtime cannot be assumed.

Candidate strategy:
package a Microsoft WebView2 **Fixed Version Runtime** beside the application and explicitly point the app at that runtime.

Microsoft documents Fixed Version as a runtime packaged with one application rather than installed system-wide. It is not automatically updated and therefore the application owner must service security/runtime updates.

Current Microsoft documentation notes that Fixed Version adds a substantial package footprint (currently more than 250 MB for runtime binaries); Tauri documentation gives a lower historical/installer increase estimate. Do not lock a size number—measure the selected current runtime during release qualification.

References:
- https://learn.microsoft.com/en-us/microsoft-edge/webview2/concepts/evergreen-vs-fixed-version
- https://learn.microsoft.com/en-us/microsoft-edge/webview2/concepts/distribution
- https://v2.tauri.app/distribute/windows-installer/

Security consequence:
Portable Offline must have an explicit WebView2 servicing policy. A fixed runtime cannot become a permanently frozen browser engine.

---

# Not required for end users

The released application must not require users to install or configure:
- Rust;
- Cargo;
- Node.js;
- npm/pnpm/yarn;
- Python;
- Visual Studio / C++ Build Tools;
- a local HTTP server;
- database server;
- browser extension;
- separate CAD/graphics software.

These may exist on development/build machines only.

---

# WebView2 runtime policy

Tauri uses Microsoft Edge WebView2 on Windows.

Current Microsoft guidance:
- Evergreen is recommended for most applications because it receives automatic security/runtime updates.
- Fixed Version is intended for constrained environments needing controlled runtime compatibility.
- a WebView2 application must ensure a suitable runtime exists by one of these strategies.

Product policy:

| Distribution | WebView2 strategy |
|---|---|
| Standard installer | Evergreen; provision missing runtime |
| Offline installer | Evergreen standalone/offline installer bundled |
| Portable Light | use compatible system Evergreen runtime |
| Portable Offline | Fixed Version candidate bundled beside app |

Do not use Tauri `skip` as a release default unless a specific artifact explicitly depends on a pre-qualified system runtime and failure behavior is tested.

---

# Application data / storage policy

Application installation location, application runtime data, user settings, caches, and project files are separate concerns.

## Canonical project files

User chooses where project files live.

Examples:
- local project folder;
- organization project directory;
- approved synced drive.

Project file location must not depend on installation mode.

Uninstalling the application must not delete normal user project files.

## App settings / cache

Use a `StoragePolicy` abstraction rather than hard-coded paths.

Installed mode:
- user-specific application-data location is preferred.

Portable mode:
- may offer app-local portable settings when the extracted directory is writable;
- may use/fallback to a user-local writable data directory for runtime/browser data when necessary.

The exact layout must be implementation-tested; "portable" primarily guarantees **no installation/admin requirement**, not necessarily zero writes outside the executable directory.

## WebView2 User Data Folder

WebView2 requires a writable User Data Folder (UDF) for browser state/cache.

Microsoft guidance says:
- a custom UDF must have read/write permission;
- WebView2 startup can fail if the UDF cannot be created/written;
- network shares are not recommended for UDF storage.

Therefore:
- explicitly choose/test a writable UDF location;
- do not assume the executable directory is always writable;
- do not place UDF on a network share as the normal supported mode;
- detect UDF startup failure and show an actionable diagnostic instead of a blank window.

Reference:
https://learn.microsoft.com/en-us/microsoft-edge/webview2/concepts/user-data-folder

---

# Office-PC / enterprise constraints

A technically portable executable may still be blocked by organization policy.

Potential controls:
- Microsoft Defender SmartScreen;
- Smart App Control;
- AppLocker / Windows Defender Application Control;
- endpoint antivirus/EDR;
- Controlled Folder Access;
- Data Loss Prevention;
- proxy/firewall policy;
- execution from removable media;
- unsigned-executable restrictions;
- restricted user-data-folder writes.

Product policy:
- no attempt to bypass enterprise controls;
- produce predictable signed artifacts and clear IT documentation;
- fail clearly when a policy blocks required files/UDF/network resources;
- core editing must not require inbound firewall ports or a localhost web server.

---

# Code signing

Status: REQUIRED FOR PUBLIC/ORGANIZATIONAL RELEASE QUALIFICATION.

Microsoft SmartScreen uses publisher/file reputation. Current Microsoft guidance recommends signing all directly distributed Windows binaries; unsigned/self-signed applications may show strong warnings and enterprise policy can block execution.

For direct distribution:
- use a trusted code-signing identity/certificate/service;
- sign every release consistently;
- sign executable and installer/portable executable artifacts as applicable;
- do not modify signed binaries afterward;
- verify signatures in release CI/qualification.

Microsoft Store distribution can be considered later, but it is not required for core product use.

References:
- https://learn.microsoft.com/en-us/windows/apps/package-and-deploy/smartscreen-reputation
- https://v2.tauri.app/distribute/

Exact signing provider/certificate mechanism is a later release-governance decision.

---

# Supported Windows target strategy

Initial product position:
- Windows x64 first.

Release qualification tiers:

## Required primary tier
- supported Windows 11 x64 office workstation.

## Compatibility tier
- Windows 10 22H2 x64 may be tested for organizations that still operate it under an appropriate support/security arrangement.

Do not promise long-term Windows 10 support solely because Tauri/WebView2 can technically run there. Product OS support should follow security/support reality and real test evidence.

## Deferred
- Windows ARM64;
- macOS;
- Linux.

Architecture may remain portable, but these are not initial release gates.

---

# Hardware target philosophy

Do not require a discrete/NVIDIA GPU for ordinary engineering work.

Target classes to qualify later:

## Minimum-office profile
- typical business x64 CPU;
- integrated Intel/AMD graphics;
- 8 GB RAM candidate minimum;
- standard SSD;
- 1080p display.

## Recommended profile
- modern business CPU;
- 16 GB RAM;
- modern integrated or discrete GPU;
- SSD;
- 1080p/1440p display.

These are **candidate test profiles, not final published system requirements**. R6–R8 performance evidence must establish actual minimum/recommended requirements.

3D presentation quality may scale with hardware, but core 2D engineering authoring must remain usable on ordinary office hardware.

---

# Network behavior

Core offline functions:
- create/open/save project;
- road/junction engineering authoring;
- 2D rendering;
- local 3D rendering;
- local assets;
- undo/redo;
- local reference images/site plans;
- export not requiring online provider content.

Optional online functions:
- online basemap/aerial providers;
- AI Copilot;
- update checking;
- remote asset/provider services later.

Network outage must not corrupt or lock local project editing.

---

# Portable mode UX

Portable mode should not require technical knowledge.

Startup may expose diagnostics only when needed:
- WebView2 missing/incompatible;
- runtime folder missing;
- data folder not writable;
- organization policy blocked execution;
- project path not writable.

Potential About/System page:
- app version;
- distribution type;
- WebView2 runtime/version;
- project schema;
- writable data path;
- renderer/GPU info;
- offline/online state.

This information is valuable for office IT support and bug reports.

---

# Release artifacts target

A qualified release should ultimately produce at least:

```text
StreetConceptDesigner_<version>_Setup.exe
StreetConceptDesigner_<version>_OfflineSetup.exe
StreetConceptDesigner_<version>_Portable.zip
StreetConceptDesigner_<version>_PortableOffline.zip
checksums.txt
release-notes
license/notices/SBOM artifacts as required
```

The exact number/names may be simplified after real deployment testing. Do not maintain variants that provide no practical value.

---

# Portable acceptance gates

Portable is not accepted because the ZIP launches on a developer computer.

Test at least:

1. clean/non-development Windows account;
2. no Administrator privilege;
3. no Node/Rust/Python installed as a prerequisite;
4. launch from normal writable local folder;
5. open/save/reopen project;
6. 2D engineering editing;
7. local 3D;
8. local reference image;
9. offline operation;
10. WebView2 behavior for the artifact's intended runtime strategy;
11. writable UDF/data behavior;
12. Unicode/Thai/space-containing path;
13. long-ish project path;
14. second launch after reboot/logoff;
15. update/replacement behavior;
16. SmartScreen/signature inspection;
17. no unexpected service/port/registry dependency for Portable;
18. no user project deletion when replacing app version.

Office-environment qualification later should additionally test:
- standard user account;
- Defender/SmartScreen;
- Controlled Folder Access if feasible;
- corporate proxy/no internet;
- IT-whitelisted signed binary path.

---

# Installer acceptance gates

1. per-user install without elevation;
2. offline installer works without internet;
3. missing WebView2 handled correctly;
4. launch after install;
5. save/open projects outside install folder;
6. uninstall does not remove project files;
7. clean reinstall;
8. signature validation;
9. upgrade from previous qualified version;
10. no unexpected machine-wide changes in per-user mode.

---

# Architecture invariants

1. Installed vs Portable changes deployment/storage policy, **not** semantic project behavior.
2. Canonical project files are portable between qualified distribution modes.
3. Core engineering model never depends on a registry-installed component other than unavoidable OS/runtime services explicitly qualified.
4. Online services are optional for core editing.
5. Fixed WebView2 runtime, if used, has a security servicing/update policy.
6. Portable packaging is tested as a product artifact rather than assumed from a bare executable.
