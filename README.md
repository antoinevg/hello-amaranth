# hello-amaranth.git

## Setup

### Dependencies

    # gtkwave
    brew install gtkwave

    # pyenv
    curl https://pyenv.run | bash

### Python Environment

    # x86_64/rosetta
    pyenv install pypy3.9-7.3.9
    pyenv virtualenv pypy3.9-7.3.9 hello-amaranth
    pyenv local hello-amaranth

    # arm64
    pyenv install 3.10.7
    pyenv virtualenv 3.10.7 hello-amaranth
    pyenv local hello-amaranth

### Amaranth

    pip install --upgrade 'amaranth[builtin-yosys]'

### Amaranth Board Definitions

    cd toolchain/
    git clone https://github.com/amaranth-lang/amaranth-boards.git
    cd amaranth-boards.git

    # fix
    pip install wheel

    python setup.py install

    # fix
    pip install markupsafe==2.0.1

### Yosys Toolchain

Grab the latest toolchain from:

    https://github.com/YosysHQ/oss-cad-suite-build/releases/latest

Copy it into the `toolchain/` directory and:n

    cd toolchain/
    tar xzf oss-cad-suite-darwin-arm64-20221031.tgz
    oss-cad-suite/activate

Activate with:

    source toolchain/oss-cad-suite/environment


## Test Installation

    python3 -m amaranth_boards.icestick


## Uninstall

    brew uninstall gtkwave

    pyenv uninstall pypy3.9-7.3.9/envs/hello-amaranth
    pyenv uninstall pypy3.9-7.3.9


## Links

* [Amaranth Documentation](https://amaranth-lang.org/docs/amaranth/latest/)
* [Building FPGA Gateware with Verilog and Amaranth: A Tutorial](https://cfu-playground.readthedocs.io/en/latest/crash-course/gateware.html)
* [fpga4fun](https://www.fpga4fun.com/)
