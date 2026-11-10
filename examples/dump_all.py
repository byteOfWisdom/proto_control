import proto_control

if __name__ == "__main__":
    uplink = proto_control.uplink("localhost", 8000, "test_recv")
    while(True):
        msg = uplink.request(mtype="data")
        print(msg)
