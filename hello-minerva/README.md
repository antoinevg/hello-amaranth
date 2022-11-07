# hello-minerva

A simple example of an Embedded Rust PAC & HAL for the Minerva RV32IM soft-core running on a @RadionaOrg #ulx3s.

## Dependencies

See [`../README.md`](../README.md) for basic repo setup


### Litex

    cd ../toolchain/litex

    wget https://raw.githubusercontent.com/enjoy-digital/litex/master/litex_setup.py
    chmod +x litex_setup.py

    ./litex_setup.py --init --install
    pip3 install git+https://github.com/litex-hub/pythondata-cpu-minerva.git


### Rust

    rustup +nightly component add rust-src


## Synthesize and load minerva bitstream

    make synth
    make svd
    make load


## Run Examples

    cd minerva-hal
    cargo run --release --example blinky


## Notes

* Rust `nightly`-only for now because `riscv32im-unknown-none-elf` is
  a [Tier
  3](https://doc.rust-lang.org/nightly/rustc/platform-support.html#tier-3)
  target which necessitates `-Z build-std=core`.


## Credits

Big shout-out to [@pepijndevos](https://github.com/pepijndevos) for his incredibly helpful blog post: [A Rust HAL for your LiteX FPGA SoC](https://pepijndevos.nl/2020/08/04/a-rust-hal-for-your-litex-fpga-soc.html)
