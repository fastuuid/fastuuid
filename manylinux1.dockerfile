FROM quay.io/pypa/manylinux1_x86_64:latest

RUN curl -sSf https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly
ENV PATH /root/.cargo/bin:$PATH
RUN /opt/python/cp38-cp38/bin/pip install maturin

RUN mkdir /tmp/src
WORKDIR /tmp/src
COPY build_or_release.sh /tmp
RUN chmod +x /tmp/build_or_release.sh
CMD ["/tmp/build_or_release.sh"]
