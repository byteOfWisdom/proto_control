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


channel_t _make_channel(size_t len) {
  channel_t res;
  res.filled = false;
  res.size = len;
  res.data = malloc(len);
  pthread_mutex_init(&res.lock, nullptr);
  return res;
}

void free_channel(channel_t* chan) {
  free(chan->data);
}


int _read_channel(channel_t* chan, void* read_to, size_t len) {  
  if (pthread_mutex_lock(&chan->lock) != 0) return -1;
  if (chan->filled) {
    memcpy(read_to, chan->data, len);
    chan->filled = false;
  }
  if (pthread_mutex_unlock(&chan->lock) != 0) return -1;
  return 0;
}


int _write_channel(channel_t* chan, void* value, size_t len) {
  if (pthread_mutex_trylock(&chan->lock) != 0) return 1;
  if (!chan->filled) {
    memcpy(chan->data, value, len);
    chan->filled = true;
  }
  if (pthread_mutex_unlock(&chan->lock) != 0) return -1;
  return 0;
}
