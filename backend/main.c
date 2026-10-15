#include <stdio.h>
#include <arpa/inet.h>
#include "server.h"

int main(int argc, char* argv[]) {
  if (argc > 1)  printf("%s\n", argv[1]);
  return 0;
}
