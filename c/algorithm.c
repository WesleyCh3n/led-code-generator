#include <stdint.h>
#include <stdlib.h>
#include <string.h>

uint8_t *rainbow(float ratio);

void deallocate_buf(uint8_t *buf) { free(buf); }

uint8_t *rainbow_buf(uint8_t len, uint8_t offset) {
  uint8_t *buffer = malloc(len * 3);
  memset(buffer, 0, len * 3);
  for (int i = 0; i < len; i++) {
    uint8_t *color = rainbow((float)i / len);
    uint8_t index = i + offset;
    index = index < len ? index : index - len;
    memcpy(buffer + (3 * index), color, 3);
    free(color);
  }
  return buffer;
}

uint8_t *rainbow(float ratio) {
  int region = ratio * 6.0;
  int normalized = ratio * 256.0 * 6.0;
  int x = normalized % 256;

  uint8_t *ptr = malloc(3);
  memset(ptr, 0, 3);

  switch (region) {
  case 0:
    ptr[0] = 255;
    ptr[1] = x;
    ptr[2] = 0;
    break;
  case 1:
    ptr[0] = 255 - x;
    ptr[1] = 255;
    ptr[2] = 0;
    break;
  case 2:
    ptr[0] = 0;
    ptr[1] = 255;
    ptr[2] = x;
    break;
  case 3:
    ptr[0] = 0;
    ptr[1] = 255 - x;
    ptr[2] = 255;
    break;
  case 4:
    ptr[0] = x;
    ptr[1] = 0;
    ptr[2] = 255;
    break;
  case 5:
    ptr[0] = 255;
    ptr[1] = 0;
    ptr[2] = 255 - x;
    break;
  }

  return ptr;
}

/*
offset should iterate between [0..2*period]
*/
uint8_t *breath_buf(uint8_t len, uint8_t period, uint8_t offset, uint8_t *c1,
                    uint8_t *c2) {
  uint8_t *buffer = malloc(len * 3);
  memset(buffer, 0, len * 3);

  uint8_t *color = malloc(3);
  offset = offset > period - 1 ? period - 1 - offset % period : offset;
  float mix = (float)offset / period;
  color[0] = c1[0] * (1 - mix) + c2[0] * mix;
  color[1] = c1[1] * (1 - mix) + c2[1] * mix;
  color[2] = c1[2] * (1 - mix) + c2[2] * mix;

  for (int i = 0; i < len; i++) {
    memcpy(buffer + (3 * i), color, 3);
  }
  free(color);
  return buffer;
}
