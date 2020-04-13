set -e

curl https://sh.rustup.rs -sSf --proto '=https' --tlsv1.2 | sh -s -- -y
$HOME/.cargo/bin/rustup install nightly-2019-09-11
$HOME/.cargo/bin/rustup default nightly-2019-09-11
wget https://repo.anaconda.com/miniconda/Miniconda3-latest-MacOSX-x86_64.sh
bash Miniconda3-latest-MacOSX-x86_64.sh  -f -b -p $HOME/miniconda3
echo "export PATH=\"\$HOME/miniconda3/bin:\$PATH\"" >> $HOME/.bashrc
echo "export PATH=\"\$HOME/miniconda3/bin:\$PATH\"" >> $HOME/.bashrc
$HOME/miniconda3/bin/conda update -y -n base -c defaults conda
$HOME/miniconda3/bin/conda init bash
$HOME/miniconda3/bin/conda create -y --name python38 python=3.8
$HOME/miniconda3/bin/conda create -y --name python37 python=3.7
$HOME/miniconda3/bin/conda create -y --name python36 python=3.6
$HOME/miniconda3/bin/conda create -y --name python35 python=3.5

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

echo twine $TWINE_USERNAME


# #####################
# Python 3.8

$HOME/miniconda3/envs/python38/bin/pip install -r requirements-dev.txt
rm -rf ./target/debug/

$HOME/.cargo/bin/cargo build
$HOME/.cargo/bin/cargo test

$HOME/miniconda3/bin/conda activate python38
$HOME/miniconda3/envs/python38/bin/maturin build --release --no-sdist -i python3.8
$HOME/miniconda3/envs/python38/bin/pip install ./target/wheels/rdp_rust-*-cp38-cp38-macosx_10_7_x86_64.whl
$HOME/miniconda3/envs/python38/bin/pytest -q test_options.py --benchmark-group-by=group

# #####################
# Python 3.7

$HOME/miniconda3/envs/python37/bin/pip install -r requirements-dev.txt
rm -rf ./target/debug/

$HOME/.cargo/bin/cargo build
$HOME/.cargo/bin/cargo test

$HOME/miniconda3/bin/conda activate python37
$HOME/miniconda3/envs/python37/bin/maturin build --release --no-sdist -i python3.7
$HOME/miniconda3/envs/python37/bin/pip install ./target/wheels/rdp_rust-*-cp37-cp37m-macosx_10_7_x86_64.whl
$HOME/miniconda3/envs/python37/bin/pytest -q test_options.py --benchmark-group-by=group

# #####################
# Python 3.6

$HOME/miniconda3/envs/python36/bin/pip install -r requirements-dev.txt
rm -rf ./target/debug/

$HOME/.cargo/bin/cargo build
$HOME/.cargo/bin/cargo test

$HOME/miniconda3/bin/conda activate python36
$HOME/miniconda3/envs/python36/bin/maturin build --release --no-sdist -i python3.6
$HOME/miniconda3/envs/python36/bin/pip install ./target/wheels/rdp_rust-*-cp36-cp36m-macosx_10_7_x86_64.whl
$HOME/miniconda3/envs/python36/bin/pytest -q test_options.py --benchmark-group-by=group

# #####################
# Upload

ls -alh ./target/wheels/
if [ -n "$TRAVIS_TAG" ]; then $HOME/miniconda3/envs/python36/bin/twine upload --skip-existing ./target/wheels/rdp_rust*.whl ; fi
