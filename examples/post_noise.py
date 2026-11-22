import proto_control
import time
import numpy as np

if __name__ == "__main__":
    uplink = proto_control.uplink("localhost", 8000, "random_data")

    start_time = time.time()
    while (True):
        x = np.random.rand()
        t = time.time() - start_time
        uplink.send_message(f"{round(t, 4)} {round(x, 4)}", "data")
        time.sleep(0.01)

