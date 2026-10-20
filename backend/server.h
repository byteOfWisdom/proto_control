#ifndef LIB_SERVER_BOUND
#define LIB_SERVER_BOUND

#include <sys/socket.h>
#include <stdio.h>
#include <stdlib.h>
#include <netinet/in.h>
#include <pthread.h>
#include <unistd.h>

typedef struct {
  int socket;
  struct sockaddr addr;
} client_desc_t;

#define CONN_BUFF_DEPTH 64

typedef struct {
  client_desc_t sockets[CONN_BUFF_DEPTH];
  int filled;
  pthread_mutex_t lock;
} connection_queue_t;

typedef struct {
  int socket;
  connection_queue_t* conn_queue;
} accept_loop_call_params_t;

int init_server(unsigned int port);
int start_accept(int socket, connection_queue_t* conn_queue);
void* accept_loop(void* params);
void* read_connection(void* params);


#endif
