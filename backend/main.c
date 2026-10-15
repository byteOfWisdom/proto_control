#include <stdio.h>
#include "server.h"

int main(int argc, char* argv[]) {
  int port = 1234;
  if (argc > 1) {
    sscanf(argv[1], "%i", &port);
  }
  printf("[INFO] Using port %d\n", port);

  int socket = init_server(port);
  if (socket < 0) {
    printf("[ERROR][main.c/main(...)] got an error initing the socket\n");
    return -1;
  }

  close(socket);
  return 0;
}
