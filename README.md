# LightCraft Studio

An open-source, sovereign RAW photo developing and color grading application built in pure Rust, powered by the **Martensite** GPU-accelerated retained-mode GUI engine.

![LightCraft Studio on Martensite](brag/demo.gif)

## Architecture

- **`crates/ui-martensite`**: Sovereign retained-mode RAW develop interface with photometric sliders, parametric tone curves, and Loupe view.
- **`crates/engine`**: Demosaicing pipeline, high-dynamic-range color management, and lossless XMP sidecar synchronization.

## Legal & Compliance Notice

LightCraft is an independent open-source photo editor. It is not affiliated with Adobe Inc. Adobe, Lightroom, and Creative Cloud are trademarks of Adobe Inc. Photometric development controls and tone curve mechanics operate under 17 U.S.C. § 102(b).

## License

Dual-licensed under MIT OR Apache-2.0.
