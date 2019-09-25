export PATH="$HOME/.cargo/bin:$PATH"
# export PATH="$HOME/miniconda/bin:$PATH"  # commented out by conda initialize

# >>> conda initialize >>>
# !! Contents within this block are managed by 'conda init' !!
__conda_setup="$('/root/miniconda/bin/conda' 'shell.bash' 'hook' 2> /dev/null)"
if [ $? -eq 0 ]; then
    eval "$__conda_setup"
else
    if [ -f "/root/miniconda/etc/profile.d/conda.sh" ]; then
        . "/root/miniconda/etc/profile.d/conda.sh"
    else
        export PATH="/root/miniconda/bin:$PATH"
    fi
fi
unset __conda_setup
# <<< conda initialize <<<

echo twine $TWINE_USERNAME

cd /root/project/

# #####################
# Python 3.7

conda activate python37

pip install -r requirements-dev.txt

cargo build
cargo test

maturin build --release --no-sdist -i python3.7
pip install /root/project/target/wheels/rdp_rust-*-cp37-cp37m-manylinux1_x86_64.whl
pytest -q test_options.py --benchmark-group-by=group

conda deactivate

# #####################
# Python 3.6

conda activate python36

pip install -r requirements-dev.txt
rm -rf target/debug/

cargo build
cargo test

maturin build --release --no-sdist -i python3.6
pip install /root/project/target/wheels/rdp_rust-*-cp36-cp36m-manylinux1_x86_64.whl
pytest -q test_options.py --benchmark-group-by=group

conda deactivate

# #####################
# Python 3.5

conda activate python35

pip install -r requirements-dev.txt
rm -rf target/debug/

cargo build
cargo test

maturin build --release --no-sdist -i python3.5
pip install /root/project/target/wheels/rdp_rust-*-cp35-cp35m-manylinux1_x86_64.whl
# Skip 3.5 due to string formatting in tests

conda deactivate

# #####################
# Upload

ls -alh ./target/wheels/

conda activate python37
if [ -n "$TRAVIS_TAG" ]; then twine upload --skip-existing ./target/wheels/rdp_rust*.whl ; fi
