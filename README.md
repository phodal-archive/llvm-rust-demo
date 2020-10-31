# llvm-rust-demo

```
brew install cmake ninja
```

```bash
cargo install llvmenv
llvmenv build-entry  10.0.0
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