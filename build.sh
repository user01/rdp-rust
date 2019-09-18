wget https://repo.anaconda.com/miniconda/Miniconda3-latest-Linux-x86_64.sh
bash Miniconda3-latest-Linux-x86_64.sh -f -b -p $HOME/miniconda
export PATH="$HOME/miniconda/bin:$PATH"
conda update -y -n base -c defaults conda
conda init bash
source $HOME/.bashrc

conda create -y --name python37 python=3.7
# conda create -y --name python36 python=3.6
# conda create -y --name python35 python=3.5

conda activate python37
# conda install -y gxx_linux-64

pip install -r requirements-dev.txt
echo Pythons
maturin list-python
rm -rf target/debug/ target/release/
maturin build --no-sdist -i python3.7
rm -rf target/debug/ target/release/
cargo test --no-default-features
pytest -q test_options.py --benchmark-group-by=group
