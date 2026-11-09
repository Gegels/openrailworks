# Performance benchmarks

Default **SLO** (Vanilla graphics profile):

- **60 FPS** target, **1920×1080**
- Measured in **Free Roam** unless noted
- Report **p50 / p95** frametime over 60 seconds

## Primary routes (project gates)

| Route | Why | TSC pain point | Engine focus |
|-------|-----|----------------|--------------|
| [Ringbahn Berlin](https://store.steampowered.com/app/2662420/) | Dense Berlin scenery | Night FPS **Tempelhof–Südkreuz** | Streaming, instancing, time-of-day |
| Südast | High object density on southern branch | Same | Same |

## Test protocol (copy into issues)

```text
OpenRailWorks version:
TSC version:
GPU / CPU:
Route:
Sim time / weather:
Start location:
GFX profile: Vanilla
FPS p50 / p95 (60s):
CPU core usage (approx):
Notes:
```

Original TSC often needs reduced **visibility** and **scenery density** on Ringbahn;
OpenRailWorks targets **full** density at the SLO above.

## Local baseline (Phase 0)

Run on your PC **before** comparing a new engine build:

1. Same route and time-of-day in **Train Simulator Classic**.
2. Repeat with **OpenRailWorks** once driving exists.
3. Store results in your fork of `COMPAT_MATRIX.md` or a private log.

`orw-catalog benchmarks` only confirms the route folders exist; it does not measure FPS yet.
