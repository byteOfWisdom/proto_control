#include <stdio.h>
#include "server.h"

#include "channel.h"


int main(int argc, char* argv[]) {
  channel_t chan = make_channel(int);
  int temp = 10;
  write_channel(&chan, &temp);
  int test;
  read_channel(&chan, &test);
  printf("test now is; %d\n", test);
  free_channel(&chan);
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
