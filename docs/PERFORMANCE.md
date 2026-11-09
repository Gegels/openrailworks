# Performance budgets (SLO)

Default product SLO — defined in `orw-engine::PerformanceSlo`:

| Metric | Vanilla profile |
|--------|-----------------|
| Resolution | 1920×1080 |
| Target FPS | 60 |
| Measurement | 60 s Free Roam, p50/p95 frametime |

## Subsystem budgets (targets for Phase 1+)

These are engineering targets, not yet enforced in code:

| Subsystem | Budget (frame @ 60 FPS) | Notes |
|-----------|-------------------------|-------|
| Simulation tick | ≤ 4 ms | Main thread; vehicles + signals |
| Lua scripts | ≤ 3 ms | Per consist; amortized |
| Streaming / IO | async on workers | No blocking main thread |
| Culling + prepare | ≤ 4 ms | Workers → render thread |
| GPU submit | ≤ 6 ms | wgpu; platform dependent |

Regression: compare **Ringbahn Berlin** and **Südast** against baseline logs in [BENCHMARKS.md](BENCHMARKS.md).
