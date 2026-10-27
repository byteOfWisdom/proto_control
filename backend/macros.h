#ifndef LIB_MACROS
#define LIB_MACROS

#define for_range(I, A, B) for (int I = A; I < B; ++I)
#define loop while(1)

#define Maybe(T) struct { \
  T content; \
  bool some; \
}

#endif
