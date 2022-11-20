# hello-lambdasoc

A simple lambdasoc SoC with peripherals.

## Dependencies

See [`../README.md`](../README.md) for basic repo setup

### LambdaSoC

    cd ../toolchain

    git clone https://github.com/lambdaconcept/lambdasoc.git lambdasoc.git
    cd lambdasoc.git

    # Don't do if we already have amaranth installed
    pip install -r requirements

    python setup.py install

### Optional: Minerva / amaranth-soc

    cd ../toolchain

    git clone https://github.com/minerva-cpu/minerva.git minerva.git
    cd minerva.git
    python setup.py install

    git clone https://github.com/amaranth-lang/amaranth-boards.git amaranth-boards.git
    cd amaranth-boards.git
    python setup.py install

    git clone https://github.com/amaranth-lang/amaranth-soc.git amaranth-soc.git
    cd amaranth-soc.git
    python setup.py install

    git clone https://github.com/amaranth-lang/amaranth-stdio.git amaranth-stdio.git
    cd amaranth-stdio.git
    python setup.py install

### Optional: Amaranth

    pip install --upgrade 'amaranth[builtin-yosys]'

    # prefer
    git clone https://github.com/amaranth-lang/amaranth.git amaranth.git
    cd amaranth.git
    python setup.py install

### Optional: Litex

    cd ../toolchain/litex

    wget https://raw.githubusercontent.com/enjoy-digital/litex/master/litex_setup.py
    chmod +x litex_setup.py

    ./litex_setup.py --init --install
    pip3 install git+https://github.com/litex-hub/pythondata-cpu-minerva.git
