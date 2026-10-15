#include <stdio.h>
#include <arpa/inet.h>
#include "server.h"

int main(int argc, char* argv[]) {
  int port = 1234;
  if (argc > 1) {
    sscanf(argv[1], "%i", &port);
  }
  printf("using port %d\n", port);


  return 0;
}
