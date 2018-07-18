FROM ubuntu:18.04

ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo
ENV PATH=/usr/local/cargo/bin:$PATH

# Use GCP apt.
RUN sed -i.bak -e "s%http://archive.ubuntu.com/ubuntu/%http://asia-northeast1.gce.archive.ubuntu.com/ubuntu/%g" /etc/apt/sources.list

# Install useful tools.
RUN apt update -qq && apt install -qqy \
        build-essential devscripts ubuntu-standard software-properties-common \
        screen lxc traceroute gdb \
        vim git subversion mercurial cmake make \
        dos2unix nkf curl xz-utils graphviz imagemagick \
        openssh-server sudo && \
    apt clean && rm -rf /var/lib/apt/lists/*
RUN mkdir -p /var/run/sshd

# Install Rust.
RUN set -eux; \
    curl -o rustup-init "https://static.rust-lang.org/rustup/archive/1.12.0/x86_64-unknown-linux-gnu/rustup-init"; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --default-toolchain 1.27.1; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; cargo --version; rustc --version

# Install scripts (python, php, ruby).
RUN apt update -qq && DEBIAN_FRONTEND=noninteractive apt install -qqy \
        php-cli php-mysql php-curl php-pear python python-pip ruby && \
    apt clean && rm -rf /var/lib/apt/lists/*

# Install C++.
RUN apt update -qq && apt install -qqy clang clang-format && \
    apt clean && rm -rf /var/lib/apt/lists/*

# Install C#.
RUN apt update -qq && apt install -qqy mono-devel && \
    apt clean && rm -rf /var/lib/apt/lists/*

# Install Java.
RUN apt update -qq && apt install -qqy default-jre default-jdk && \
    apt clean && rm -rf /var/lib/apt/lists/*

# Install Bazel.
RUN echo "deb [arch=amd64] http://storage.googleapis.com/bazel-apt stable jdk1.8" | \
    tee /etc/apt/sources.list.d/bazel.list
RUN curl https://bazel.build/bazel-release.pub.gpg | apt-key add -
RUN apt update -qq && apt install -qqy bazel && \
    apt clean && rm -rf /var/lib/apt/lists/*

# Set up CircleCI configs.
RUN groupadd --gid 3434 circleci \
  && useradd --uid 3434 --gid circleci --shell /bin/bash --create-home circleci \
  && echo 'circleci ALL=NOPASSWD: ALL' >> /etc/sudoers.d/50-circleci \
  && echo 'Defaults    env_keep += "DEBIAN_FRONTEND"' >> /etc/sudoers.d/env_keep

USER circleci

CMD ["/bin/bash"]
