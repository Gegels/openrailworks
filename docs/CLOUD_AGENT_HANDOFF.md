# Übergabe an einen neuen Cloud-Agenten

**Repo:** https://github.com/Gegels/openrailworks  
**Zweck:** Train Simulator Classic neu implementieren (OpenOMSI-Stil) — eigene Engine, **keine** Spielinhalte im Repo.

---

## Was der neue Agent wissen muss (Kurzfassung)

1. **Lesen:** [AGENTS.md](../AGENTS.md), [README.md](../README.md), [ROADMAP.md](ROADMAP.md), [BENCHMARKS.md](BENCHMARKS.md).
2. **Nicht tun:** TSC/Dovetail-Binaries oder DLC-Assets committen; Installation des Users nur **read-only** (`RAILWORKS_PATH`).
3. **Stand v0.0.1:** Rust-Monorepo mit `orw-catalog` (Strecken-Scanner), `orw-engine` (wgpu-Vorbereitung), CI Win/Linux/macOS.
4. **Nordstern:** Alle Strecken des Users **flüssig** (60 FPS @ 1080p Vanilla); harte Tests **Ringbahn Berlin** + **Südast** (Nacht Tempelhof–Südkreuz).
5. **Plattformen:** Windows, Linux, macOS (wgpu).

---

## Erste Nachricht an den neuen Agenten (copy & paste)

```text
Projekt: OpenRailWorks (openOMSI für Train Simulator Classic).
Repo-Root ist ein Rust-Monorepo. Lies AGENTS.md und docs/ROADMAP.md.

Aktuell: Phase 1 starten — VFS für RailWorks-Installation (.ap + lose Dateien),
danach minimaler Route-Loader (Tracks.bin) und Rendering-Grundgerüst (wgpu).
Benchmark-Strecken: Ringbahn Berlin, Südast (docs/BENCHMARKS.md).

Regeln:
- Keine Spielinhalte ins Git
- AGPL-3.0, TRADEMARKS.md beachten
- Kleine PRs, COMPAT_MATRIX pflegen
- cargo test --workspace vor jedem Commit

Meine TSC-Installation liegt lokal unter [PFAD EINFÜGEN]; in der Cloud nur Mocks/Tests ohne echte Assets.
```

Ersetze `[PFAD EINFÜGEN]` durch deinen Steam-`RailWorks`-Ordner (z. B. `C:\Program Files (x86)\Steam\steamapps\common\RailWorks`).

---

## Monorepo (wo was liegt)

| Pfad | Inhalt |
|------|--------|
| `crates/orw-core` | Install-Erkennung, Route-Catalog |
| `crates/orw-catalog-cli` | CLI `orw-catalog` |
| `crates/orw-engine` | Engine-Skelett, Performance-SLO |
| `docs/` | Roadmap, Benchmarks, Legal, Matrix |
| `.github/workflows/ci.yml` | Build/Test alle OS |
| `scripts/build-*.sh` | Release-Builds |

---

## Sinnvolle Reihenfolge (Phase 1)

| Schritt | Aufgabe | Erfolg |
|---------|---------|--------|
| 1 | `crates/orw-vfs` — Pfad + `.ap`-Lesen (read-only) | Unit-Tests mit Fake-Archiv |
| 2 | `Tracks.bin` / Blueprint-Stubs in `docs/FORMATS.md` | Parser lädt Metadaten |
| 3 | wgpu: Fenster oder Headless-Smoke in CI | Grün auf 3 OS |
| 4 | Erste Strecke nur **anzeigen** (kein Fahren) | Mit User-Installation lokal getestet |
| 5 | Free Roam + eine Lok + Lua minimal | Ringbahn-Performance messen |

Details: [ROADMAP.md](ROADMAP.md).

---

## Lokal bei dir (Georg)

```bash
git clone https://github.com/Gegels/openrailworks.git
cd openrailworks
cargo build --release -p orw-catalog

# Windows PowerShell:
$env:RAILWORKS_PATH = "C:\Program Files (x86)\Steam\steamapps\common\RailWorks"
.\target\release\orw-catalog.exe doctor
.\target\release\orw-catalog.exe routes
.\target\release\orw-catalog.exe benchmarks
```

Ergebnisse (FPS, Frametimes) für Ringbahn/Südast in Issues oder `docs/COMPAT_MATRIX.md` — **ohne** Screenshots mit fremden DLC-Rechten, wenn unsicher.

---

## Git / Cloud Agent

- **Primary remote:** `origin` → GitHub `Gegels/openrailworks`.
- Push: `./scripts/push-github.sh` (braucht `GH_TOKEN` oder Deploy Key auf GitHub).
- `.cursor/environment.json`: Rust-Toolchain für Agent-Builds.

Wenn der Agent aus **GitHub** gestartet wird, hat er den Code aus dem Repo — **nach dem ersten Push** von `main` inkl. `AGENTS.md` und dieser Datei.

---

## Community-Alternative

Parallel prüfen: Community-Engine **NEXT** (Rail-Sim.de) gegen dieselben Strecken — ggf. Mitentwicklung statt Doppelarbeit. Eigenes Repo bleibt OpenRailWorks.
