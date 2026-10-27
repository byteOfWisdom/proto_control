#ifndef LIB_CHANNEL
#define LIB_CHANNEL

#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
  void* data;
  size_t size;
  bool filled;
  pthread_mutex_t lock;
} channel_t;

typedef channel_t* channel_ref_t;

#define make_channel(type) _make_channel(sizeof(type))
#define read_channel(C, B) _read_channel(C, (void*) B, (C)->size) 
#define write_channel(C, V) _write_channel(C, (void*) V, (C)->size) 

void free_channel(channel_t* chan);
channel_t _make_channel(size_t len);
int _read_channel(channel_t* chan, void* read_to, size_t len);
int _write_channel(channel_t* chan, void* value, size_t len);

#endif
