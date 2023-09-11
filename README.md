# Nexodia
![Static Badge](https://img.shields.io/badge/Windows-%E2%9C%94-green?logo=windows&logoColor=white&link=http://a.com/)
![Static Badge](https://img.shields.io/badge/Linux-%E2%9A%99-blue?logo=linux&logoColor=white&link=http://a.com/)
![Static Badge](https://img.shields.io/badge/Mac-%E2%9C%96-red?logo=apple&logoColor=white&link=http://a.com/)
![Static Badge](https://img.shields.io/badge/Android-%E2%9C%96-red?logo=android&logoColor=white&link=http://a.com/)

An 3D MMO RPG written in Rust using the [WGPU](https://wgpu.rs/) crate.

Currently aiming only for Windows, but Linux and Android support is planned.

### 🚀 Running
```shell
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
    "compression_level": 12, /* ZSTD compression level, from 1 to 22 */
    /* Mesh settings */
    "uv": true,
    "normal": true,
    /* Texture settings */
    "pixel_opacity": true
}
```

## License
Feel free to use the code as you want.