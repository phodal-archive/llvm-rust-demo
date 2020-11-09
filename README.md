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


### run & build

```
LLVM_SYS_100_PREFIX=$HOME/llvm/llvm-10.0.1.src/build cargo build
LLVM_SYS_100_PREFIX=$HOME/llvm/llvm-10.0.1.src/build cargo run
````

### docs

```bash
?> 1 + 1
=> 2

?> var a = 5, b = 10 in a * b
=> 50

?> def fib(n) if n < 2 then n else fib(n - 1) + fib(n - 2)

?> fib(40)
=> 102334155

?>
```

## Documents

related: [LLVM tutorial in the Rust language](https://github.com/jauhien/iron-kaleidoscope)
