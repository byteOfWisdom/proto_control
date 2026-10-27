import proto_control
from time import sleep

if __name__ == "__main__":
    uplink = proto_control.uplink("localhost", 8000, "test_ping")

    i = 0
    while (True):
        uplink.send_message(f"test {i}", "ping")
        sleep(1)
        i += 1
