# Nexodia

An 3D MMO RPG written in Rust using the [WGPU](https://wgpu.rs/) crate.

Currently aiming only for Windows, but Linux and Android support is planned.

### 🚀 Running
```console
~$ blender --background --python compiler.py
~$ cargo run --bin compiler
~$ cargo run --bin nexodia
```

### 📑 Todo
- [x] Instances
- [x] Textures
- [ ] Animations
- [ ] Shadows

## Compiler

### 📁 Organization.
- All assets should be in the [assets](./assets/) folder.
- Compiled binaries go to the same folder as the source, using the <b>.bin</b> extension.
- The compiler binary will compile all <b>.glb</b> and <b>.gltf</b> files as meshes, and all <b>.png</b>, <b>.jpg</b> and <b>.jpeg</b> files as textures.
- The [compiler.py](./compiler/compiler.py) Blender script will compile all <b>.fbx</b> files as animations.

### ⚙ Settings
Settings files should have the settings.json name and will apply to its folder and children.

These are the available settings:
```json
{
    "compression_level": 12, // ZSTD compression level, from 1 to 22
    // Mesh settings
    "uv": true,
    "normal": true,
    // Texture settings
    "pixel_opacity": true
}
```