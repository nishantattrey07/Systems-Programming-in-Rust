import socket
import time

HOST = "127.0.0.1"
PORT = 8080

passed = 0
failed = []


def separator(title):
    print("\n" + "=" * 70)
    print(title)
    print("=" * 70)


def receive(timeout=1):
    s.settimeout(timeout)

    chunks = []

    try:
        while True:
            data = s.recv(4096)

            if not data:
                break

            chunks.append(data.decode())

            if len(data) < 4096:
                break

    except socket.timeout:
        pass

    return "".join(chunks).strip()


def run_test(name, fn):
    global passed, failed

    separator(name)

    try:
        fn()
        passed += 1
        print(f"✅ {name} PASSED")

    except AssertionError as e:
        failed.append((name, str(e)))
        print(f"❌ {name} FAILED")
        print(e)

    except Exception as e:
        failed.append((name, str(e)))
        print(f"💥 {name} CRASHED")
        print(e)


def assert_response(command, expected, sleep=0.1):
    print(f"\nSending: {repr(command)}")

    s.send(command)

    time.sleep(sleep)

    actual = receive()

    assert actual == expected, (
        f"\nExpected:\n{repr(expected)}"
        f"\nGot:\n{repr(actual)}"
    )

    print(f"Response: {repr(actual)}")


def send_partial(data, sleep=0.1):
    print(f"Sending partial: {repr(data)}")
    s.send(data)
    time.sleep(sleep)


def assert_after_partial(expected):
    actual = receive()

    assert actual == expected, (
        f"\nExpected:\n{repr(expected)}"
        f"\nGot:\n{repr(actual)}"
    )

    print(f"Response: {repr(actual)}")


s = socket.socket()
s.connect((HOST, PORT))

separator("CONNECTED")


# ---------------------------------------------------
# TESTS
# ---------------------------------------------------

run_test(
    "TEST 1 — Valid SET",
    lambda: assert_response(
        b"SET name nishant\n",
        "OK"
    )
)

run_test(
    "TEST 2 — Valid GET",
    lambda: assert_response(
        b"GET name\n",
        "VALUE nishant"
    )
)

run_test(
    "TEST 3 — Valid DELETE",
    lambda: assert_response(
        b"DELETE name\n",
        "OK"
    )
)

run_test(
    "TEST 4 — GET deleted key",
    lambda: assert_response(
        b"GET name\n",
        "NOT_FOUND"
    )
)


def test_multiple_commands():
    s.send(
        b"SET city delhi\nGET city\nDELETE city\n"
    )

    time.sleep(0.2)

    actual = receive().split("\n")

    expected = [
        "OK",
        "VALUE delhi",
        "OK"
    ]

    assert actual == expected, (
        f"\nExpected:\n{expected}"
        f"\nGot:\n{actual}"
    )


run_test(
    "TEST 5 — Multiple commands in one chunk",
    test_multiple_commands
)


def test_fragmented():
    send_partial(b"SET ")
    send_partial(b"framework ")
    send_partial(b"rust\n")

    assert_after_partial("OK")


run_test(
    "TEST 6 — Fragmented command",
    test_fragmented
)


def test_full_partial():
    send_partial(b"SET x 10\nGET")

    assert_after_partial("OK")

    send_partial(b" x\n")

    assert_after_partial("VALUE 10")


run_test(
    "TEST 7 — Full + partial command",
    test_full_partial
)


def test_byte_fragmentation():
    for ch in b"SET language rust\n":
        s.send(bytes([ch]))
        time.sleep(0.02)

    assert_after_partial("OK")


run_test(
    "TEST 8 — One byte at a time",
    test_byte_fragmentation
)


def test_value_spaces():
    assert_response(
        b"SET bio i love rust systems programming\n",
        "OK"
    )

    assert_response(
        b"GET bio\n",
        "VALUE i love rust systems programming"
    )


run_test(
    "TEST 9 — Value with spaces",
    test_value_spaces
)


def test_weird_spacing():
    assert_response(
        b"SET      spaced      value\n",
        "OK"
    )

    # preserved whitespace in value
    assert_response(
        b"GET spaced\n",
        "VALUE      value"
    )


run_test(
    "TEST 10 — Weird spacing",
    test_weird_spacing
)

run_test(
    "TEST 11 — Unknown command",
    lambda: assert_response(
        b"HELLO world\n",
        "ERR unknown command <HELLO>"
    )
)

run_test(
    "TEST 12 — Missing SET value",
    lambda: assert_response(
        b"SET key\n",
        "ERR missing value"
    )
)

run_test(
    "TEST 13 — Missing GET key",
    lambda: assert_response(
        b"GET\n",
        "ERR missing key"
    )
)

run_test(
    "TEST 14 — Missing DELETE key",
    lambda: assert_response(
        b"DELETE\n",
        "ERR missing key"
    )
)

run_test(
    "TEST 15 — GET extra args",
    lambda: assert_response(
        b"GET name extra\n",
        "ERR invalid command format <GET name extra>"
    )
)

run_test(
    "TEST 16 — DELETE extra args",
    lambda: assert_response(
        b"DELETE age extra\n",
        "ERR invalid command format <DELETE age extra>"
    )
)

run_test(
    "TEST 17 — Empty command",
    lambda: assert_response(
        b"\n",
        "ERR empty command"
    )
)

run_test(
    "TEST 18 — Lowercase command",
    lambda: assert_response(
        b"set framework rust\n",
        "OK"
    )
)

run_test(
    "TEST 19 — Mixed case command",
    lambda: assert_response(
        b"SeT language rust\n",
        "OK"
    )
)


def test_disconnect():
    s.send(b"SET unfinished")
    time.sleep(0.5)


run_test(
    "TEST 20 — Disconnect mid-command",
    test_disconnect
)

separator("SUMMARY")

print(f"Passed: {passed}")
print(f"Failed: {len(failed)}")

if failed:
    print("\nFAILED TESTS:")

    for name, error in failed:
        print(f"\n{name}")
        print(error)
else:
    print("\n🎉 ALL TESTS PASSED")

s.close()