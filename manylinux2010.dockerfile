FROM quay.io/pypa/manylinux2010_x86_64:latest

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly
ENV PATH /root/.cargo/bin:$PATH
RUN rustup install nightly
RUN rustup default nightly
RUN /opt/python/cp37-cp37m/bin/pip install pyo3-pack

RUN mkdir /tmp/src
WORKDIR /tmp/src
COPY build_or_release.sh /tmp
RUN chmod +x /tmp/build_or_release.sh
CMD ["/tmp/build_or_release.sh"]
