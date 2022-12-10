#include "src.h"

int main(int argc, char *argv[]) {
  int n = argc > 1 ? atoi(argv[1]) : 100;

  double left = calc(101);

  double right = -18.67;
  if (fabs(left - right) > 0.1) {
    fprintf(stderr, "%f != %f\n", left, right);
    exit(EXIT_FAILURE);
  }

  double results = calc(n);

  printf("%f\n", results);
}