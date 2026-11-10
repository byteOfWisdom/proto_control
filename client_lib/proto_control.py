import socket
from types import NoneType

class uplink:
    def __init__(self, ip, port, sender_name):
        self._ip = ip
        self._port = port
        self._name = sender_name
        self._socket = None
        self._last_id = 0
        self._connect()
        self.type_dict = {
            "misc": "M",
            "query": "Q",
            "command": "C",
            "data": "D",
            "invalid": "I",
            
        }

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
        type_str = self.type_dict[type] if type in self.type_dict else "I"
        msg_string = f"{self._name}|{type_str}|{message}\0"
        not_sent = True
        while (not_sent):
            try:
                self._socket.sendall(bytes(msg_string, "utf-8"))
                not_sent = False
            except BrokenPipeError:
                self._connect()
        return 0


    def request(self, from_filter="*", mtype="misc", content_filter="*", min_id=None):
        if isinstance(min_id, NoneType):
            min_id = self._last_id
        type_str = self.type_dict[mtype] if mtype in self.type_dict else "I"
        self.send_message(f"sender_cond={from_filter};min_id={min_id};content_cond={content_filter};mtype_cond={type_str}", type="query")

        response = ""
        while "\0" not in response:

            # TODO: make this more efficient
            chunk = self._socket.recv(1024)
            response += chunk.decode("utf-8")

        msg_parts = response.split("|")
        response_msg = {
            "from": msg_parts[0],
            "mtype": msg_parts[1], # TODO: convert to string
            "id": int(msg_parts[2]),
            "content": "".join(msg_parts[3:])
        }

        if response_msg["mtype"] != "I":
            self._last_id = response_msg["id"] + 1
            return response_msg

        return None
