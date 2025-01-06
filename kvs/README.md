# KVS

A simple key-value store to follow the course
from [Pingcap](https://github.com/pingcap/talent-plan/blob/master/courses/rust).

## Development

Make sure you have Rust installed. Ideally, you should use Nix along with
nix-direnv, so that related libraries (OpenSSL and pkg-config) are installed 
automatically.

Here is an example on the environment setup from nix-direnv:

```
direnv: loading .../kvs/.envrc
direnv: using nix
direnv: nix-direnv: Using cached dev shell
direnv: export +AR +AS +CC +CONFIG_SHELL +CXX +HOST_PATH +IN_NIX_SHELL +LD +NIX_BINTOOLS +NIX_BINTOOLS_WRAPPER_TARGET_HOST_x86_64_unknown_linux_gnu +NIX_BUILD_CORES +NIX_CC +NIX_CC_WRAPPER_TARGET_HOST_x86_64_unknown_linux_gnu +NIX_CFLAGS_COMPILE +NIX_ENFORCE_NO_NATIVE +NIX_HARDENING_ENABLE +NIX_LDFLAGS +NIX_PKG_CONFIG_WRAPPER_TARGET_HOST_x86_64_unknown_linux_gnu +NIX_STORE +NM +OBJCOPY +OBJDUMP +PKG_CONFIG +PKG_CONFIG_PATH +RANLIB +READELF +SIZE +SOURCE_DATE_EPOCH +STRINGS +STRIP +__structuredAttrs +buildInputs +buildPhase +builder +cmakeFlags +configureFlags +depsBuildBuild +depsBuildBuildPropagated +depsBuildTarget +depsBuildTargetPropagated +depsHostHost +depsHostHostPropagated +depsTargetTarget +depsTargetTargetPropagated +doCheck +doInstallCheck +dontAddDisableDepTrack +mesonFlags +name +nativeBuildInputs +out +outputs +patches +phases +preferLocalBuild +propagatedBuildInputs +propagatedNativeBuildInputs +shell +shellHook +stdenv +strictDeps +system ~PATH ~XDG_DATA_DIRS
```

---

- Start the IDE:

```shell
rust-rover > /dev/null 2>&1 &!
```

We do this to make that the IDE is inheriting the environment variables from 
the shell. Without the inheritance, the shell within the IDE won't be able to
build the project.

- Build:

```shell
cargo build
```

- Test:

```shell
cargo test
```

- Benchmark:

```shell
cargo bench
```
