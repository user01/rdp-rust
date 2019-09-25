FROM ubuntu:bionic-20190807

RUN apt update
RUN apt install -y curl build-essential wget

RUN curl https://sh.rustup.rs -sSf --proto '=https' --tlsv1.2 | sh -s -- -y
RUN cat $HOME/.cargo/env >> $HOME/.bashrc
RUN /root/.cargo/bin/rustup install nightly-2019-09-11
RUN /root/.cargo/bin/rustup default nightly-2019-09-11

RUN wget https://repo.anaconda.com/miniconda/Miniconda3-latest-Linux-x86_64.sh
RUN bash Miniconda3-latest-Linux-x86_64.sh -f -b -p $HOME/miniconda
RUN echo "export PATH=\"\$HOME/miniconda/bin:\$PATH\"" >> $HOME/.bashrc

RUN $HOME/miniconda/bin/conda update -y -n base -c defaults conda
RUN $HOME/miniconda/bin/conda init bash
RUN $HOME/miniconda/bin/conda create -y --name python37 python=3.7
RUN $HOME/miniconda/bin/conda create -y --name python36 python=3.6
RUN $HOME/miniconda/bin/conda create -y --name python35 python=3.5

WORKDIR /root/project

# docker build -f Dockerfile -t build-img-temp .
# docker run --rm -it -v $PWD:/root/project/ build-img-temp bash /root/project/build.sh