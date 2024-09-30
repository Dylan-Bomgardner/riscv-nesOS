// extern "C" void kmain(void);
#include "kmain.h"
typedef unsigned int uint32_t;

extern "C" uint32_t bss_start;
extern "C" uint32_t bss_end;
extern "C" uint32_t data_start;
extern "C" uint32_t data_end;
extern "C" uint32_t rodata_start;
extern "C" uint32_t rodata_end;

void print_char(char c) { 
  char* uart = (char*)0x10000000;
  *uart = c;
}

void print_string(const char* str) {
  while(*str) {
    print_char(*str);
    str++;
  }
}

int main(void) {
  print_string("hello world");

  while (1);

  return 0;
}