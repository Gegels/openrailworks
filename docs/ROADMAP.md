# Roadmap (openOMSI-style phases)

| Phase | Focus | Success |
|-------|--------|---------|
| **0** | Baseline vs TSC on Ringbahn + Südast | Frametime notes in [BENCHMARKS.md](BENCHMARKS.md) |
| **0.0.1** (now) | Install discovery, route catalog CLI, CI | `orw-catalog` + docs |
| **1** | First drivable slice: one route, Free Roam, minimal Lua | 60 FPS @ 1080p Vanilla SLO on reference hardware |
| **2** | All routes in **your** install: load + Free Roam + SLO | [COMPAT_MATRIX.md](COMPAT_MATRIX.md) 100 % owned routes |
| **3** | Workshop / third-party packs from issues | Matrix grows with community |
| **4** | Scenarios, signals, braking | Ringbahn scenarios playable |
| **5** | Enhanced graphics (opt-in), editor, multiplayer | Optional |

## Technical milestones

1. **VFS** — unified read path for loose files and `.ap` archives.
2. **Streaming** — tile/scenery jobs on worker threads; main thread simulation.
3. **wgpu renderer** — Vanilla profile first; Enhanced later.
4. **Lua host** — incremental API surface matching vehicle scripts.
5. **Launcher** — route picker, Stable/Beta channel from GitHub Releases.

See [FORMATS.md](FORMATS.md) for format documentation (stubs welcome).
