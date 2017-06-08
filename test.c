#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <sys/time.h>

#define NUM_PIXELS (1000*100000) // 100 MP image

// CAM to RGB matrix taken from the Sony A77V
float matrix[3][4] = {
  {0.75925,      0.21533987,  0.020964727, 0},
  {0.28515315,   1.0129657,  -0.2981189,   0},
  {0.021620918, -0.19883351,  1.177143,    0},
};

int64_t time_in_micros () {
  struct timeval tv;
  gettimeofday(&tv, NULL);

  return tv.tv_sec*1000000+tv.tv_usec;
}

float incr(float val, float incr) {
  float newval = val+incr;

  if (newval > 1.0f)
    return 0.0f;

  return newval;
}

int main(void) {
  float *in = (float *) malloc(NUM_PIXELS*4*sizeof(float));
  float *out = (float *) malloc(NUM_PIXELS*3*sizeof(float));

  // Fill in the input buffer with different rgb values
  float r = 0.0f;
  float g = 0.0f;
  float b = 0.0f;
  for(int i=0; i<NUM_PIXELS; i++) {
    r = incr(r, 0.01);
    g = incr(g, 0.02);
    b = incr(b, 0.03);

    in[i*4+0] = r;
    in[i*4+1] = g;
    in[i*4+2] = b;
    in[i*4+3] = 0;
  }

  int64_t from_time = time_in_micros();
  for(int i=0; i<NUM_PIXELS; i++) {
    out[i*3+0] = in[i*4+0] * matrix[0][0] + in[i*4+1] * matrix[0][1] + in[i*4+2] * matrix[0][2] + in[i*4+3] * matrix[0][3];
    out[i*3+1] = in[i*4+0] * matrix[1][0] + in[i*4+1] * matrix[1][1] + in[i*4+2] * matrix[1][2] + in[i*4+3] * matrix[1][3],
    out[i*3+2] = in[i*4+0] * matrix[2][0] + in[i*4+1] * matrix[2][1] + in[i*4+2] * matrix[2][2] + in[i*4+3] * matrix[2][3];
  }
  int64_t to_time = time_in_micros();

  // Calculate the pixel average
  double sum = 0.0f;
  for(int i=0; i<NUM_PIXELS*3; i++) {
    sum += out[i];
  }

  printf("%0.2f ms/megapixel (sum is %f)\n", 
         (to_time - from_time)/(NUM_PIXELS/1000.0),
         sum);
}
