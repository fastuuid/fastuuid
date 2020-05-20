#!/bin/sh

if [[ -z "${TRAVIS_TAG}" ]]; then
  /opt/python/cp38-cp38/bin/maturin build --manylinux $MANYLINUX_VERSION --release -i /opt/python/cp38-cp38/bin/python -i /opt/python/cp37-cp37m/bin/python -i /opt/python/cp36-cp36m/bin/python -i /opt/python/cp35-cp35m/bin/python
  ls -1 target/wheels/*.whl | xargs -I%  auditwheel show %
else
  /opt/python/cp38-cp38/bin/maturin publish ${NO_SDIST:+--no-sdist} --username $PYPI_USERNAME --manylinux $MANYLINUX_VERSION -i /opt/python/cp38-cp38/bin/python -i /opt/python/cp37-cp37m/bin/python -i /opt/python/cp36-cp36m/bin/python -i /opt/python/cp35-cp35m/bin/python
fi
