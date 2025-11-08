# Just IR loader

Impulse response loader without any additional functionality

To build run:

`cargo xtask bundle just_ir_loader --release`

On Mac OS you might need to run:

`xattr -rd com.apple.quarantine target/bundled/just_ir_loader.vst3/`

after building to allow plugin to be inserted to your DAW
