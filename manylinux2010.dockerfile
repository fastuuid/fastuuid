FROM quay.io/pypa/manylinux2010_x86_64:latest

RUN curl -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly
ENV PATH /root/.cargo/bin:$PATH
RUN rustup install nightly
RUN rustup default nightly
RUN /opt/python/cp37-cp37m/bin/pip install --pre maturin

RUN mkdir /tmp/src
WORKDIR /tmp/src
COPY build_or_release.sh /tmp
RUN chmod +x /tmp/build_or_release.sh
CMD ["/tmp/build_or_release.sh"]
