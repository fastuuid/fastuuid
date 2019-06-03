import re
import uuid

import pytest
from fastuuid import UUID, uuid3, uuid4, uuid5
from hypothesis import given
from hypothesis.strategies import uuids

UUID_REGEX = re.compile("[0-F]{8}-([0-F]{4}-){3}[0-F]{12}", re.I)


def test_uuid_without_arguments():
    with pytest.raises(TypeError,
                       match="one of the hex, bytes, bytes_le, fields, or int arguments must be given"):
        UUID()


@given(uuids())
def test_hex(expected):
    expected = str(expected)

    assert str(UUID(hex=expected)) == expected


def test_hex_bad_string():
    with pytest.raises(ValueError,
                       match="badly formed hexadecimal UUID string"):
        UUID("bla")


@given(uuids())
def test_int(expected):
    assert str(UUID(int=expected.int)) == str(expected)


@given(uuids())
def test_bytes(expected):
    assert str(UUID(bytes=expected.bytes)) == str(expected)


@given(uuids())
def test_bytes_le(expected):
    assert str(UUID(bytes_le=expected.bytes_le)) == str(expected)


def test_equality():
    expected = uuid4()
    actual = UUID(str(expected))
    other = uuid4()

    assert expected == actual
    assert expected != other


def test_comparision():
    a = UUID(int=10)
    b = UUID(int=20)
    c = UUID(int=20)

    assert a < b
    assert b > a
    assert a <= b
    assert b >= a
    assert c <= c
    assert c >= c


@pytest.mark.xfail(raises=NotImplementedError)
@given(uuids())
def test_fields(expected):
    assert str(UUID(fields=expected.fields)) == str(expected)


@given(uuids())
def test_int_property(u):
    expected = u.int
    actual = UUID(str(u)).int

    assert expected == actual


@given(uuids())
def test_bytes_property(u):
    expected = u.bytes
    actual = UUID(str(u)).bytes()

    assert expected == actual


@given(uuids())
def test_hex_property(u):
    expected = u.hex
    actual = UUID(str(u)).hex

    assert expected == actual


@given(uuids())
def test_urn_property(u):
    expected = u.urn
    actual = UUID(str(u)).urn

    assert expected == actual


@given(uuids())
def test_bytes_le_property(u):
    expected = u.bytes_le
    actual = UUID(str(u)).bytes_le()

    assert expected == actual


def test_uuid3():
    expected = uuid3(uuid4(), b"foo")
    assert UUID_REGEX.match(str(expected))
    assert str(expected) == str(uuid.UUID(str(expected)))


def test_uuid4():
    expected = uuid4()
    assert UUID_REGEX.match(str(expected))
    assert str(expected) == str(uuid.UUID(str(expected)))


def test_uuid5():
    expected = uuid5(uuid4(), b"foo")
    assert UUID_REGEX.match(str(expected))
    assert str(expected) == str(uuid.UUID(str(expected)))
