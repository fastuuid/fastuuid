import re
from fastuuid import uuid3, uuid4, UUID
import uuid
import pytest

UUID_REGEX = re.compile("[0-F]{8}-([0-F]{4}-){3}[0-F]{12}", re.I)


def test_uuid_without_arguments():
    with pytest.raises(TypeError,
                       match="one of the hex, bytes, bytes_le, fields, or int arguments must be given"):
        UUID()


def test_hex():
    expected = str(uuid.uuid4())

    assert str(UUID(hex=expected)) == expected


def test_hex_bad_string():
    with pytest.raises(ValueError,
                       match="badly formed hexadecimal UUID string"):
        UUID("bla")


def test_int():
    expected = uuid.uuid4()

    assert str(UUID(int=expected.int)) == str(expected)


def test_bytes():
    expected = uuid.uuid4()

    assert str(UUID(bytes=expected.bytes)) == str(expected)


def test_bytes_le():
    expected = uuid.uuid4()

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
def test_fields():
    expected = uuid.uuid4()

    assert str(UUID(fields=expected.fields)) == str(expected)


def test_uuid3():
    expected = uuid3(uuid4(), b"foo")
    assert UUID_REGEX.match(str(expected))
