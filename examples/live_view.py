import proto_control
import numpy as np
from sys import argv
from matplotlib import pyplot as plt
import io
import matplotlib.animation as animation

ip = argv[2] if len(argv) > 2 else "localhost"
uplink = proto_control.uplink(ip, 8080, "live_view")

x, y = np.array([]), np.array([])
fig, ax = plt.subplots()
line, = ax.plot(x, y)
max_len = 1000


def update_data(_):
    global x, y
    msg = uplink.request(mtype="data", from_filter=from_filter)
    max_events = 1000
    ec = 0
    while msg and ec < max_events:
        inc_x = float(msg["content"].split()[0])
        inc_y = float(msg["content"].split()[1])
        x = np.append(x, inc_x)
        y = np.append(y, inc_y)
        msg = uplink.request(mtype="data", from_filter=from_filter)
        ec += 1

    if len(x) > max_len:
        x = x[-max_len:]
        y = y[-max_len:]
    line.set_data(x, y)
    ax.set(xlim=(min(x), max(x)), ylim=(min(y), max(y)))
    return line
    


if __name__ == "__main__":
    from_filter = argv[1]

    ani = animation.FuncAnimation(fig=fig, func=update_data, frames=None, interval=1, cache_frame_data=False)
    plt.show()
