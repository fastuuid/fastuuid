from concurrent.futures import ThreadPoolExecutor
from uuid import UUID, uuid3, uuid4, uuid5

import pytest
from fastuuid import UUID as FastUUID
from fastuuid import uuid3 as fastuuid3
from fastuuid import uuid4 as fastuuid4
from fastuuid import uuid4_bulk, uuid4_as_strings_bulk
from fastuuid import uuid5 as fastuuid5


@pytest.fixture(scope="session")
def example():
    return uuid4()


@pytest.mark.benchmark(group='parse-hex')
def test_parse_hex_uuid(benchmark, example):
    benchmark(UUID, hex=str(example))


@pytest.mark.benchmark(group='parse-hex')
def test_parse_hex_fastuuid(benchmark, example):
    benchmark(FastUUID, hex=str(example))


@pytest.mark.benchmark(group='parse-bytes')
def test_parse_bytes_uuid(benchmark, example):
    benchmark(UUID, bytes=example.bytes)


@pytest.mark.benchmark(group='parse-bytes')
def test_parse_bytes_fastuuid(benchmark, example):
    benchmark(FastUUID, bytes=example.bytes)


@pytest.mark.benchmark(group='parse-bytes_le')
def test_parse_bytes_le_uuid(benchmark, example):
    benchmark(UUID, bytes_le=example.bytes_le)


@pytest.mark.benchmark(group='parse-bytes_le')
def test_parse_bytes_le_fastuuid(benchmark, example):
    benchmark(FastUUID, bytes_le=example.bytes_le)


@pytest.mark.benchmark(group='parse-fields')
def test_parse_fields_uuid(benchmark, example):
    benchmark(UUID, fields=example.fields)


@pytest.mark.benchmark(group='parse-fields')
def test_parse_fields_fastuuid(benchmark, example):
    benchmark(FastUUID, fields=example.fields)


@pytest.mark.benchmark(group='parse-int')
def test_parse_int_uuid(benchmark, example):
    benchmark(UUID, int=example.int)


@pytest.mark.benchmark(group='parse-int')
def test_parse_int_fastuuid(benchmark, example):
    benchmark(FastUUID, int=example.int)


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


@pytest.mark.benchmark(group='uuidv4 8 threads - strings')
def test_uuidv4_str_threads(benchmark):
    @benchmark
    def b():
        pool = ThreadPoolExecutor(max_workers=8)
        for _ in range(8):
            pool.submit(lambda: [str(uuid4()) for _ in range(1000)])
        pool.shutdown(wait=True)


@pytest.mark.benchmark(group='uuidv4 8 threads - strings')
def test_fast_uuidv4_str_threads(benchmark):
    @benchmark
    def b():
        pool = ThreadPoolExecutor(max_workers=8)
        for _ in range(8):
            pool.submit(lambda: [str(fastuuid4()) for _ in range(1000)])
        pool.shutdown(wait=True)


@pytest.mark.benchmark(group='uuidv4 8 threads - strings')
def test_fast_uuidv4_bulk_convert_to_strings_threads(benchmark):
    @benchmark
    def b():
        pool = ThreadPoolExecutor(max_workers=8)
        for _ in range(8):
            pool.submit(lambda: map(str, uuid4_bulk(1000)))
        pool.shutdown(wait=True)


@pytest.mark.benchmark(group='uuidv4 8 threads - strings')
def test_fast_uuidv4_as_strings_bulk_threads(benchmark):
    @benchmark
    def b():
        pool = ThreadPoolExecutor(max_workers=8)
        for _ in range(8):
            pool.submit(uuid4_as_strings_bulk, 1000)
        pool.shutdown(wait=True)
