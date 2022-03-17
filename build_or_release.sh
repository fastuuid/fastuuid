#!/bin/sh

if [[ -z "${TRAVIS_TAG}" ]]; then
  /opt/python/cp39-cp39/bin/maturin build --compatibility $MANYLINUX_VERSION --release -i /opt/python/cp310-cp310/bin/python -i /opt/python/cp39-cp39/bin/python -i /opt/python/cp38-cp38/bin/python -i /opt/python/cp37-cp37m/bin/python
  ls -1 target/wheels/*.whl | xargs -I%  auditwheel show %
else
  /opt/python/cp39-cp39/bin/maturin publish ${NO_SDIST:+--no-sdist} --username $PYPI_USERNAME --compatibility $MANYLINUX_VERSION -i /opt/python/cp310-cp310/bin/python -i /opt/python/cp39-cp39/bin/python -i /opt/python/cp38-cp38/bin/python -i /opt/python/cp37-cp37m/bin/python
fi
