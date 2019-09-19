brew update
brew install gcc curl wget
curl https://sh.rustup.rs -sSf --proto '=https' --tlsv1.2 | sh -s -- -y
$HOME/.cargo/bin/rustup default nightly
wget https://repo.anaconda.com/miniconda/Miniconda3-latest-MacOSX-x86_64.sh
bash Miniconda3-latest-MacOSX-x86_64.sh  -f -b -p $HOME/miniconda
echo "export PATH=\"\$HOME/miniconda/bin:\$PATH\"" >> $HOME/.bashrc
echo "export PATH=\"\$HOME/miniconda/bin:\$PATH\"" >> $HOME/.bashrc
$HOME/miniconda/bin/conda update -y -n base -c defaults conda
$HOME/miniconda/bin/conda init bash
$HOME/miniconda/bin/conda create -y --name python37 python=3.7
$HOME/miniconda/bin/conda create -y --name python36 python=3.6
$HOME/miniconda/bin/conda create -y --name python35 python=3.5

export PATH="$HOME/.cargo/bin:$PATH"
# export PATH="$HOME/miniconda/bin:$PATH"  # commented out by conda initialize

# added by Miniconda3 4.5.12 installer
# >>> conda init >>>
# !! Contents within this block are managed by 'conda init' !!
__conda_setup="$(CONDA_REPORT_ERRORS=false '/Users/travis/miniconda3/bin/conda' shell.bash hook 2> /dev/null)"
if [ $? -eq 0 ]; then
    \eval "$__conda_setup"
else
    if [ -f "/Users/travis/miniconda3/etc/profile.d/conda.sh" ]; then
        . "/Users/travis/miniconda3/etc/profile.d/conda.sh"
        CONDA_CHANGEPS1=false conda activate base
    else
        \export PATH="/Users/travis/miniconda3/bin:$PATH"
    fi
fi
unset __conda_setup
# <<< conda init <<<



# #####################
# Python 3.7

conda activate python37

pip install -r requirements-dev.txt

cargo build
cargo test

maturin build --no-sdist -i python3.7
pip install ./target/wheels/rdp_rust-*-cp37-cp37m-macosx_10_7_x86_64.whl
pytest -q test_options.py --benchmark-group-by=group

conda deactivate

# #####################
# Python 3.6

conda activate python36

pip install -r requirements-dev.txt
rm -rf target/debug/

cargo build
cargo test

maturin build --no-sdist -i python3.6
pip install ./target/wheels/rdp_rust-*-cp36-cp36m-macosx_10_7_x86_64.whl
pytest -q test_options.py --benchmark-group-by=group

conda deactivate

# #####################
# Python 3.5

conda activate python35

pip install -r requirements-dev.txt
rm -rf target/debug/

cargo build
cargo test

maturin build --no-sdist -i python3.5
pip install ./target/wheels/rdp_rust-*-cp35-cp35m-macosx_10_7_x86_64.whl
# Skip 3.5 due to string formatting in tests

conda deactivate