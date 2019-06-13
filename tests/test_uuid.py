import re
import uuid

import pytest
from fastuuid import UUID, uuid3, uuid4, uuid5
from hypothesis import given
from hypothesis.strategies import binary, integers, lists, text, tuples, uuids

UUID_REGEX = re.compile("[0-F]{8}-([0-F]{4}-){3}[0-F]{12}", re.I)


def test_uuid_without_arguments():
    with pytest.raises(TypeError,
                       match="one of the hex, bytes, bytes_le, fields, or int arguments must be given"):
        UUID()


@given(uuids())
def test_hex(expected):
    expected = str(expected)

    assert str(UUID(hex=expected)) == expected


@given(text())
def test_hex_bad_string(bad_hex):
    with pytest.raises(ValueError,
                       match="badly formed hexadecimal UUID string"):
        UUID(bad_hex)


@given(binary().filter(lambda x: len(x) != 16))
def test_bad_bytes(bad_bytes):
    with pytest.raises(ValueError,
                       match="bytes is not a 16-char string"):
        UUID(bytes=bad_bytes)


@given(binary().filter(lambda x: len(x) != 16))
def test_bad_bytes_le(bad_bytes):
    with pytest.raises(ValueError,
                       match="bytes_le is not a 16-char string"):
        UUID(bytes_le=bad_bytes)


@given(expected=uuids(),
       bad_version=integers(min_value=5, max_value=20))
def test_bad_version(expected, bad_version):
    with pytest.raises(ValueError,
                       match="illegal version number"):
        UUID(str(expected), version=10)


@given(lists(integers(), max_size=10).filter(lambda x: len(x) != 6))
def test_wrong_fields_count(fields):
    fields = tuple(fields)
    with pytest.raises(ValueError,
                       match="fields is not a 6-tuple"):
        UUID(fields=fields)


@given(tuples(integers(min_value=1 << 32),
              integers(min_value=0, max_value=1 << 16 - 1),
              integers(min_value=0, max_value=1 << 16 - 1),
              integers(min_value=0, max_value=1 << 8 - 1),
              integers(min_value=0, max_value=1 << 8 - 1),
              integers(min_value=0, max_value=1 << 48 - 1)))
def test_fields_time_low_out_of_range(fields):
    with pytest.raises(ValueError,
                       match=r"field 1 out of range \(need a 32-bit value\)"):
        UUID(fields=fields)


@given(tuples(integers(min_value=0, max_value=1 << 32 - 1),
              integers(min_value=1 << 16),
              integers(min_value=0, max_value=1 << 16 - 1),
              integers(min_value=0, max_value=1 << 8 - 1),
              integers(min_value=0, max_value=1 << 8 - 1),
              integers(min_value=0, max_value=1 << 48 - 1)))
def test_fields_time_mid_out_of_range(fields):
    with pytest.raises(ValueError,
                       match=r"field 2 out of range \(need a 16-bit value\)"):
        UUID(fields=fields)


@given(tuples(integers(min_value=0, max_value=1 << 32 - 1),
              integers(min_value=0, max_value=1 << 16 - 1),
              integers(min_value=1 << 16),
              integers(min_value=0, max_value=1 << 8 - 1),
              integers(min_value=0, max_value=1 << 8 - 1),
              integers(min_value=0, max_value=1 << 48 - 1)))
def test_fields_time_high_version_out_of_range(fields):
    with pytest.raises(ValueError,
                       match=r"field 3 out of range \(need a 16-bit value\)"):
        UUID(fields=fields)


@given(tuples(integers(min_value=0, max_value=1 << 32 - 1),
              integers(min_value=0, max_value=1 << 16 - 1),
              integers(min_value=0, max_value=1 << 16 - 1),
              integers(min_value=1 << 8),
              integers(min_value=0, max_value=1 << 8 - 1),
              integers(min_value=0, max_value=1 << 48 - 1)))
def test_fields_clock_seq_hi_variant_out_of_range(fields):
    with pytest.raises(ValueError,
                       match=r"field 4 out of range \(need a 8-bit value\)"):
        UUID(fields=fields)


@given(tuples(integers(min_value=0, max_value=1 << 32 - 1),
              integers(min_value=0, max_value=1 << 16 - 1),
              integers(min_value=0, max_value=1 << 16 - 1),
              integers(min_value=0, max_value=1 << 8 - 1),
              integers(min_value=1 << 8),
              integers(min_value=0, max_value=1 << 48 - 1)))
def test_fields_clock_seq_low_out_of_range(fields):
    with pytest.raises(ValueError,
                       match=r"field 5 out of range \(need a 8-bit value\)"):
        UUID(fields=fields)


@given(tuples(integers(min_value=0, max_value=1 << 32 - 1),
              integers(min_value=0, max_value=1 << 16 - 1),
              integers(min_value=0, max_value=1 << 16 - 1),
              integers(min_value=0, max_value=1 << 8 - 1),
              integers(min_value=0, max_value=1 << 8 - 1),
              integers(min_value=1 << 48)))
def test_fields_node_out_of_range(fields):
    with pytest.raises(ValueError,
                       match=r"field 6 out of range \(need a 48-bit value\)"):
        UUID(fields=fields)


@given(uuids())
def test_int(expected):
    actual = UUID(int=expected.int)
    assert str(actual) == str(expected)
    assert int(actual) == int(expected)


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
    actual = UUID(str(u)).bytes

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
    actual = UUID(str(u)).bytes_le

    assert expected == actual


@given(uuids())
def test_fields_property(u):
    expected = u.fields
    actual = UUID(str(u)).fields

    assert expected == actual


@given(uuids())
def test_time_low_property(u):
    expected = u.time_low
    actual = UUID(str(u)).time_low

    assert expected == actual


@given(uuids())
def test_time_mid_property(u):
    expected = u.time_mid
    actual = UUID(str(u)).time_mid

    assert expected == actual


@given(uuids())
def test_time_hi_version_property(u):
    expected = u.time_hi_version
    actual = UUID(str(u)).time_hi_version

    assert expected == actual


@given(uuids())
def test_clock_seq_hi_variant_property(u):
    expected = u.clock_seq_hi_variant
    actual = UUID(str(u)).clock_seq_hi_variant

    assert expected == actual


@given(uuids())
def test_clock_seq_low_property(u):
    expected = u.clock_seq_low
    actual = UUID(str(u)).clock_seq_low

    assert expected == actual


@given(uuids())
def test_time_property(u):
    expected = u.time
    actual = UUID(str(u)).time

    assert expected == actual


@given(uuids())
def test_node_property(u):
    expected = u.node
    actual = UUID(str(u)).node

    assert expected == actual


def test_uuid3():
    expected = uuid3(uuid4(), b"foo")
    assert expected.version == 3
    assert expected.variant == "specified in RFC 4122"
    assert UUID_REGEX.match(str(expected))
    assert str(expected) == str(uuid.UUID(str(expected)))


def test_uuid4():
    expected = uuid4()
    assert expected.version == 4
    assert expected.variant == "specified in RFC 4122"
    assert UUID_REGEX.match(str(expected))
    assert str(expected) == str(uuid.UUID(str(expected)))


def test_uuid5():
    expected = uuid5(uuid4(), b"foo")
    assert expected.version == 5
    assert expected.variant == "specified in RFC 4122"
    assert UUID_REGEX.match(str(expected))
    assert str(expected) == str(uuid.UUID(str(expected)))
