import copy
from concurrent.futures import ThreadPoolExecutor
from pickle import dumps, loads
from random import getrandbits
from uuid import UUID, uuid1, uuid3, uuid4, uuid5

import pytest
from uuid_extensions import uuid7, uuid7str

from fastuuid import UUID as FastUUID
from fastuuid import uuid1 as fastuuid1
from fastuuid import uuid3 as fastuuid3
from fastuuid import uuid4 as fastuuid4
from fastuuid import uuid4_as_strings_bulk, uuid4_bulk
from fastuuid import uuid5 as fastuuid5
from fastuuid import uuid7 as fastuuid7
from fastuuid import uuid7_as_strings_bulk, uuid7_bulk
from fastuuid import uuid_v1mc as fast_uuid_v1mc


def uuid_v1mc():
    """Reference implementation of uuid_v1mc()."""
    return uuid1(getrandbits(48) | (1 << 40), getrandbits(14))


def uuid_v1mc():
    """Reference implementation of uuid_v1mc()."""
    return uuid1(getrandbits(48) | (1 << 40), getrandbits(14))


def uuid_v1mc():
    """Reference implementation of uuid_v1mc()."""
    return uuid1(getrandbits(48) | (1 << 40), getrandbits(14))


@pytest.fixture(scope="session")
def example():
    return uuid4()


@pytest.fixture(scope="session")
def example_fastuuid():
    return fastuuid4()


@pytest.mark.benchmark(group="parse-hex")
def test_parse_hex_uuid(benchmark, example):
    benchmark(UUID, hex=str(example))


@pytest.mark.benchmark(group="parse-hex")
def test_parse_hex_fastuuid(benchmark, example):
    benchmark(FastUUID, hex=str(example))


@pytest.mark.benchmark(group="parse-bytes")
def test_parse_bytes_uuid(benchmark, example):
    benchmark(UUID, bytes=example.bytes)


@pytest.mark.benchmark(group="parse-bytes")
def test_parse_bytes_fastuuid(benchmark, example):
    benchmark(FastUUID, bytes=example.bytes)


@pytest.mark.benchmark(group="parse-bytes_le")
def test_parse_bytes_le_uuid(benchmark, example):
    benchmark(UUID, bytes_le=example.bytes_le)


@pytest.mark.benchmark(group="parse-bytes_le")
def test_parse_bytes_le_fastuuid(benchmark, example):
    benchmark(FastUUID, bytes_le=example.bytes_le)


@pytest.mark.benchmark(group="parse-fields")
def test_parse_fields_uuid(benchmark, example):
    benchmark(UUID, fields=example.fields)


@pytest.mark.benchmark(group="parse-fields")
def test_parse_fields_fastuuid(benchmark, example):
    benchmark(FastUUID, fields=example.fields)


@pytest.mark.benchmark(group="parse-int")
def test_parse_int_uuid(benchmark, example):
    benchmark(UUID, int=example.int)


@pytest.mark.benchmark(group="parse-int")
def test_parse_int_fastuuid(benchmark, example):
    benchmark(FastUUID, int=example.int)


@pytest.mark.benchmark(group="uuidv1")
def test_uuidv1(benchmark):
    benchmark(uuid1)


@pytest.mark.benchmark(group="uuidv1")
def test_fast_uuidv1(benchmark):
    benchmark(fastuuid1)


@pytest.mark.benchmark(group="uuid_v1mc")
def test_uuidv1mc(benchmark):
    benchmark(uuid_v1mc)


@pytest.mark.benchmark(group="uuid_v1mc")
def test_fast_uuidv1mc(benchmark):
    benchmark(fast_uuid_v1mc)


@pytest.mark.benchmark(group="uuidv1_params")
@pytest.mark.parametrize(
    ("node", "clock_seq"),
    [
        pytest.param(123, 456, id="node and clock_seq"),
        pytest.param(123, None, id="node"),
        pytest.param(None, 456, id="clock_seq"),
    ],
)
def test_uuidv1_custom(benchmark, node, clock_seq):
    benchmark(uuid1, node, clock_seq)


@pytest.mark.benchmark(group="uuidv1_params")
@pytest.mark.parametrize(
    ("node", "clock_seq"),
    [
        pytest.param(123, 456, id="node and clock_seq"),
        pytest.param(123, None, id="node"),
        pytest.param(None, 456, id="clock_seq"),
    ],
)
def test_fast_uuidv1_custom(benchmark, node, clock_seq):
    benchmark(fastuuid1, node, clock_seq)


@pytest.mark.benchmark(group="uuidv3")
def test_uuidv3(benchmark):
    benchmark(uuid3, uuid4(), "benchmark")


@pytest.mark.benchmark(group="uuidv3")
def test_fast_uuidv3(benchmark):
    benchmark(fastuuid3, fastuuid4(), b"benchmark")


@pytest.mark.benchmark(group="uuidv4")
def test_uuidv4(benchmark):
    benchmark(uuid4)


@pytest.mark.benchmark(group="uuidv4")
def test_fast_uuidv4(benchmark):
    benchmark(fastuuid4)


@pytest.mark.benchmark(group="uuidv5")
def test_uuidv5(benchmark):
    benchmark(uuid5, uuid4(), "benchmark")


@pytest.mark.benchmark(group="uuidv5")
def test_fast_uuidv5(benchmark):
    benchmark(fastuuid5, fastuuid4(), b"benchmark")


@pytest.mark.benchmark(group="uuidv7")
def test_uuidv7(benchmark):
    benchmark(uuid7)


@pytest.mark.benchmark(group="uuidv7")
def test_fast_uuidv7(benchmark):
    benchmark(fastuuid7)


@pytest.mark.benchmark(group="uuidv4 8 threads")
def test_uuidv4_threads(benchmark):
    @benchmark
    def b():
        pool = ThreadPoolExecutor(max_workers=8)
        for _ in range(8):
            pool.submit(lambda: [uuid4() for _ in range(1000)])
        pool.shutdown(wait=True)


@pytest.mark.benchmark(group="uuidv4 8 threads")
def test_fast_uuidv4_threads(benchmark):
    @benchmark
    def b():
        pool = ThreadPoolExecutor(max_workers=8)
        for _ in range(8):
            pool.submit(lambda: [fastuuid4() for _ in range(1000)])
        pool.shutdown(wait=True)


@pytest.mark.benchmark(group="uuidv4 8 threads")
def test_fast_uuidv4_bulk_threads(benchmark):
    @benchmark
    def b():
        pool = ThreadPoolExecutor(max_workers=8)
        for _ in range(8):
            pool.submit(uuid4_bulk, 1000)
        pool.shutdown(wait=True)


@pytest.mark.benchmark(group="uuidv4 8 threads - strings")
def test_uuidv4_str_threads(benchmark):
    @benchmark
    def b():
        pool = ThreadPoolExecutor(max_workers=8)
        for _ in range(8):
            pool.submit(lambda: [str(uuid4()) for _ in range(1000)])
        pool.shutdown(wait=True)


@pytest.mark.benchmark(group="uuidv4 8 threads - strings")
def test_fast_uuidv4_str_threads(benchmark):
    @benchmark
    def b():
        pool = ThreadPoolExecutor(max_workers=8)
        for _ in range(8):
            pool.submit(lambda: [str(fastuuid4()) for _ in range(1000)])
        pool.shutdown(wait=True)


@pytest.mark.benchmark(group="uuidv4 8 threads - strings")
def test_fast_uuidv4_bulk_convert_to_strings_threads(benchmark):
    @benchmark
    def b():
        pool = ThreadPoolExecutor(max_workers=8)
        for _ in range(8):
            pool.submit(lambda: map(str, uuid4_bulk(1000)))
        pool.shutdown(wait=True)


@pytest.mark.benchmark(group="uuidv4 8 threads - strings")
def test_fast_uuidv4_as_strings_bulk_threads(benchmark):
    @benchmark
    def b():
        pool = ThreadPoolExecutor(max_workers=8)
        for _ in range(8):
            pool.submit(uuid4_as_strings_bulk, 1000)
        pool.shutdown(wait=True)


@pytest.mark.benchmark(group="uuidv7 8 threads")
def test_uuidv7_threads(benchmark):
    @benchmark
    def b():
        pool = ThreadPoolExecutor(max_workers=8)
        for _ in range(8):
            pool.submit(lambda: [uuid7() for _ in range(1000)])
        pool.shutdown(wait=True)


@pytest.mark.benchmark(group="uuidv7 8 threads")
def test_fast_uuidv7_threads(benchmark):
    @benchmark
    def b():
        pool = ThreadPoolExecutor(max_workers=8)
        for _ in range(8):
            pool.submit(lambda: [fastuuid7() for _ in range(1000)])
        pool.shutdown(wait=True)


@pytest.mark.benchmark(group="uuidv7 8 threads")
def test_fast_uuidv7_bulk_threads(benchmark):
    @benchmark
    def b():
        pool = ThreadPoolExecutor(max_workers=8)
        for _ in range(8):
            pool.submit(uuid7_bulk, 1000)
        pool.shutdown(wait=True)


@pytest.mark.benchmark(group="uuidv7 8 threads - strings")
def test_uuidv7_str_threads(benchmark):
    @benchmark
    def b():
        pool = ThreadPoolExecutor(max_workers=8)
        for _ in range(8):
            pool.submit(lambda: [str(uuid7()) for _ in range(1000)])
        pool.shutdown(wait=True)


@pytest.mark.benchmark(group="uuidv7 8 threads - strings")
def test_fast_uuidv7_str_threads(benchmark):
    @benchmark
    def b():
        pool = ThreadPoolExecutor(max_workers=8)
        for _ in range(8):
            pool.submit(lambda: [str(fastuuid7()) for _ in range(1000)])
        pool.shutdown(wait=True)


@pytest.mark.benchmark(group="uuidv7 8 threads - strings")
def test_fast_uuidv7_bulk_convert_to_strings_threads(benchmark):
    @benchmark
    def b():
        pool = ThreadPoolExecutor(max_workers=8)
        for _ in range(8):
            pool.submit(lambda: map(str, uuid7_bulk(1000)))
        pool.shutdown(wait=True)


@pytest.mark.benchmark(group="uuidv7 8 threads - strings")
def test_fast_uuidv7_as_strings_bulk_threads(benchmark):
    @benchmark
    def b():
        pool = ThreadPoolExecutor(max_workers=8)
        for _ in range(8):
            pool.submit(uuid7_as_strings_bulk, 1000)
        pool.shutdown(wait=True)


def _pickle_unpickle(u):
    return loads(dumps(u))


@pytest.mark.benchmark(group="pickle")
def test_pickle_unpickle(benchmark, example):
    benchmark(_pickle_unpickle, example)


@pytest.mark.benchmark(group="pickle")
def test_pickle_unpickle_fastuuid(benchmark, example_fastuuid):
    benchmark(_pickle_unpickle, example_fastuuid)


@pytest.mark.benchmark(group="copy")
def test_copy(benchmark, example):
    benchmark(lambda u: copy.copy(u), example)


@pytest.mark.benchmark(group="copy")
def test_copy_fastuuid(benchmark, example_fastuuid):
    benchmark(lambda u: copy.copy(u), example_fastuuid)


@pytest.mark.benchmark(group="deep-copy")
def test_deep_copy(benchmark, example):
    benchmark(lambda u: copy.deepcopy(u), example)


@pytest.mark.benchmark(group="deep-copy")
def test_deep_copy_fastuuid(benchmark, example_fastuuid):
    benchmark(lambda u: copy.deepcopy(u), example_fastuuid)
