#include <stdio.h>
#include "server.h"
#include "channel.h"
#include "macros.h"

int main(int argc, char* argv[]) {
  if (argc == 1) {
    printf("USAGE: ./run [port number]\n");
    return 0;
  }

  int port = 1234; // bad default port. change to reasonable default or require port argument
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
