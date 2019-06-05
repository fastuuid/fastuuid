FROM quay.io/pypa/manylinux2010_x86_64:latest

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH /root/.cargo/bin:$PATH
RUN rustup install nightly
RUN rustup default nightly
RUN /opt/python/cp37-cp37m/bin/pip install pyo3-pack

RUN mkdir /tmp/src
WORKDIR /tmp/src

CMD ["/opt/python/cp37-cp37m/bin/pyo3-pack", "build", "--manylinux", "2010", "--release", "-i", "/opt/python/cp37-cp37m/bin/python", "-i", "/opt/python/cp36-cp36m/bin/python", "-i", "/opt/python/cp35-cp35m/bin/python"]
