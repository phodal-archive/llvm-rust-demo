# llvm-rust-demo

```
brew install cmake ninja
```

## Install LLVM


### remote

```bash
cargo install llvmenv
```

usage

```
llvmenv init
llvmenv entries
local-llvm 10.0.0
```

about `1.5 hours` in my MBP 2018


```
LLVM_SYS_100_PREFIX=$HOME/llvm/llvm-10.0.1.src cargo build
````

### local

Add local llvm for build to: `$XDG_CONFIG_HOME/llvmenv/entry.toml

docs: https://docs.rs/llvmenv/0.3.0/llvmenv/entry/index.html

````
[local-llvm]
path = "/path/to/your/src"
target = ["X86"]
```

**build**

```
llvmenv build-entry local-llvm
```


## Documents

### install for llvm 10.0

for error:

```bash
error: No suitable version of LLVM was found system-wide or pointed
       to by LLVM_SYS_100_PREFIX.

       Consider using `llvmenv` to compile an appropriate copy of LLVM, and
       refer to the llvm-sys documentation for more information.

       llvm-sys: https://crates.io/crates/llvm-sys
       llvmenv: https://crates.io/crates/llvmenv
```