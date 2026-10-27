#include "server.h"
#include <stddef.h>
#include <stdio.h>

int init_server(unsigned int port) {
  // returns an initialized and bound socket. -1 on any failure.

  int sock = socket(AF_INET, SOCK_STREAM, 0);
  if (sock < 0) {
    return -1;
  }

  struct sockaddr_in address;
  address.sin_family = AF_INET;
  address.sin_addr.s_addr = INADDR_ANY;
  address.sin_port = htons(port);

  if (bind(sock, (struct sockaddr*) &address, sizeof(address)) < 0) {
    return -1;
  }
  
  return sock;
}

int start_accept(int socket, connection_queue_t* conn_queue){
    conn_queue->filled = 0;
    pthread_mutex_init(&conn_queue->lock, nullptr);
    pthread_t thread_handle;
    accept_loop_call_params_t params = {
      .socket = socket,
      .conn_queue = conn_queue,
    };
    if (pthread_create(&thread_handle, nullptr, &accept_loop, (void*) &params) != 0) {
      fprintf(stderr, "[ERROR][server.v/start_accept(...)] got an error on pthread_create\n");
      return -1;
    }

    return 0;
}


void* accept_loop(void* params_raw) {
  accept_loop_call_params_t* params = (accept_loop_call_params_t*) params_raw;
  int socket = params->socket;
  connection_queue_t* conn_queue = params->conn_queue;

  // expects conn_queue to be set up and socket to be set up and bound
  // the actual accept loop which will lock the mutex, accept a connection, unlock the mutex

  while (1) {
    client_desc_t new_client;
    socklen_t addr_len = sizeof(new_client.socket);
    if ((new_client.socket = accept(socket, &new_client.addr, &addr_len)) < 0) {
      fprintf(stderr, "[ERROR][server.h/accept_loop(...)] got an error trying to accept a socket connection\n");
    } else {
      conn_queue->filled += 1;
    }

    while (pthread_mutex_lock(&conn_queue->lock)) {
      fprintf(stderr, "[ERROR][server.h/accept_loop(...)] got an error locking the mutex\n");
      sleep(1);
    }
    
    if (conn_queue->filled >= CONN_BUFF_DEPTH) {
      fprintf(stderr, "[WARN][server.h/accept_loop(...)] found a full connect buffer. closing last connection.\n");
      close(new_client.socket);
    } else {
      conn_queue->sockets[conn_queue->filled++] = new_client;
    }

    if (pthread_mutex_unlock(&conn_queue->lock)) {
      fprintf(stderr, "[ERROR][server.h/accept_loop(...)] got an error unlocking the mutex\n");
    }
    
  }
  return nullptr;
}


message_t alloc_message(size_t content_size) {
  message_t message;
  message.content = (char*) calloc(content_size, sizeof(char));
  if (message.content == nullptr) {
    fprintf(stderr, "[ERROR][server.c/alloc_message] failed to allocate memory for message\n");
  }

  return message;
}


void free_message(message_t* message) {
  free(message->content);
  return;
}


maybe_message_t read_socket(int socket) {
  maybe_message_t res;
  res.some = false;
  return res;
}
