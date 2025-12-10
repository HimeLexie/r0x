<pre>
|‾‾‾‾\‾‾\   /‾/ 
| |/ /‾\ \ / /  
| r-⟨ 0 ⟩ x ⟨     - A RUST HEX DUMPER
| |\ \_/ / \ \  
|_| \___/   \_\
</pre>
___

This is a little hex dumper I made as a practice project to get back into coding, but I really liked the final product so I decided to publish it here. Enjoy!

## Build Instructions
### Linux
Simply install rustup toolchain via [the rustup toolchain manager/installer](https://rustup.rs/), then run `./build.sh linux`.
### Windows
Same as Linux except the command is `./build.sh windows`. A good way to run bash scripts on Windows is git bash through [Git for Windows](https://git-scm.com/install/windows).
### Cross-compilation
To cross-compile windows from a linux device you should install your package manager's package for `llvm`, `wine`, and `lld`. Then, run `rustup target add x86_64-pc-windows-msvc` to get the msvc toolchain. You should then install xwin via `cargo install xwin` and run from **this** directory `xwin --accept-license splate --output .xwin`. Lastly, run `./build.sh`.

example:
![example](https://files.catbox.moe/0tv0e1.png)
