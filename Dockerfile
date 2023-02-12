FROM 'centos:8'

RUN yum install -y sudo gcc
RUN yum install -y iputils bind-utils make

ARG USER
ARG UID
RUN groupadd ${USER}
RUN useradd -d /home/${USER} -m -s /bin/bash -u ${UID} -g ${USER} ${USER}
USER ${USER}

RUN curl https://sh.rustup.rs -sSf >> ${HOME}/rustup.rs
RUN sh ${HOME}/rustup.rs -y
RUN echo $HOME
ENV PATH=/home/${USER}/.cargo/bin:$PATH
RUN echo $PATH

WORKDIR '/work'