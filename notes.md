# Breaktimer usage in comparison mac
- 261mb when break is on
- 156 when break is off



# Licensing
- licensing automation or frequently run `cargo bundle-licenses --format yaml --output THIRDPARTY.yml`
 - this is also useful `cargo license`
 - it's also good to check `find ~/.cargo/registry -iname "NOTICE*"` every now and then for NOTICES



# Linux testing on mac
To run checks (cli and within lsp) for linux `target_os` on my macbook, I had to install `libxscrnsaver`, `libx11`, and `pkg-config`. I also needed to insall the specfic linux target using `rustup`, and use the following configuration: 

``` toml
# .cargo/config.toml

[build]
target = "aarch64-unknown-linux-gnu"

[env]
PKG_CONFIG_SYSROOT_DIR= "/"

```

