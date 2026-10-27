import socket

class uplink:
    def __init__(self, ip, port, sender_name):
        self._ip = ip
        self._port = port
        self._name = sender_name
        self._socket = None
        self._connect()

    def _connect(self):
        self._socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        try:
            self._socket.connect((self._ip, self._port))
        except Exception:
            return

    def send_message(self, message, type="misc"):
        if "\0" in message:
            # todo: more professional errors
            print("DONT PUT ZERO BYTES INTO MESSAGE STRINGS")
            print("NOT SENDING THAT")
            print("bYEEEE")
            return 1
        msg_string = f"{self._name}|{type}|{message}\0"
        not_sent = True
        while (not_sent):
            try:
                self._socket.sendall(bytes(msg_string, "utf-8"))
                not_sent = False
            except BrokenPipeError:
                self._connect()
        return 0
        
