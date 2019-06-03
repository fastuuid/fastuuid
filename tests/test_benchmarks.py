from concurrent.futures import ThreadPoolExecutor
from uuid import uuid3, uuid4, uuid5

import pytest
from fastuuid import uuid3 as fastuuid3
from fastuuid import uuid4 as fastuuid4
from fastuuid import uuid4_bulk
from fastuuid import uuid5 as fastuuid5


@pytest.mark.benchmark(group='uuidv3')
def test_uuidv3(benchmark):
    benchmark(uuid3, uuid4(), "benchmark")


@pytest.mark.benchmark(group='uuidv3')
def test_fast_uuidv3(benchmark):
    benchmark(fastuuid3, fastuuid4(), b"benchmark")


@pytest.mark.benchmark(group='uuidv4')
def test_uuidv4(benchmark):
    benchmark(uuid4)


@pytest.mark.benchmark(group='uuidv4')
def test_fast_uuidv4(benchmark):
    benchmark(fastuuid4)


@pytest.mark.benchmark(group='uuidv5')
def test_uuidv5(benchmark):
    benchmark(uuid5, uuid4(), "benchmark")


@pytest.mark.benchmark(group='uuidv5')
def test_fast_uuidv5(benchmark):
    benchmark(fastuuid5, fastuuid4(), b"benchmark")


@pytest.mark.benchmark(group='uuidv4 8 threads')
def test_uuidv4_threads(benchmark):
    @benchmark
    def b():
        pool = ThreadPoolExecutor(max_workers=8)
        for _ in range(8):
            pool.submit(lambda: [uuid4() for _ in range(1000)])
        pool.shutdown(wait=True)


@pytest.mark.benchmark(group='uuidv4 8 threads')
def test_fast_uuidv4_threads(benchmark):
    @benchmark
    def b():
        pool = ThreadPoolExecutor(max_workers=8)
        for _ in range(8):
            pool.submit(lambda: [fastuuid4() for _ in range(1000)])
        pool.shutdown(wait=True)


@pytest.mark.benchmark(group='uuidv4 8 threads')
def test_fast_uuidv4_bulk_threads(benchmark):
    @benchmark
    def b():
        pool = ThreadPoolExecutor(max_workers=8)
        for _ in range(8):
            pool.submit(uuid4_bulk, 1000)
        pool.shutdown(wait=True)
