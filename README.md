# Nexodia
![Static Badge](https://img.shields.io/badge/Windows-%E2%9C%94-green?logo=windows&logoColor=white)
![Static Badge](https://img.shields.io/badge/Linux-%E2%9A%99-blue?logo=linux&logoColor=white)
![Static Badge](https://img.shields.io/badge/Mac-%E2%9C%96-red?logo=apple&logoColor=white)
![Static Badge](https://img.shields.io/badge/Android-%E2%9C%96-red?logo=android&logoColor=white)

An 3D MMO RPG written in Rust using the [WGPU](https://wgpu.rs/) crate.

Currently aiming only for Windows, but Linux and Android support is planned.

### 🚀 Running
```shell
blender --background --python compiler/utils/install_deps.py
./compiler/compile.ssh
cargo run --bin nexodia
```

### 📑 Todo
- [x] Instances
- [x] Textures
- [x] Animations
- [x] Shadows
- [ ] Cascaded shadows
- [ ] Global illumination
- [ ] Gather all events between frames and join, send only one to script per frame