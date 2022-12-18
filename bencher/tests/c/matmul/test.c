#include "src.h"

int main(int argc, char *argv[]) {
  int size = argc > 1 ? atoi(argv[1]) : 100;
  printf("%d\n", size);

  double left = calc(101);

  double right = -18.67;
  if (fabs(left - right) > 0.1) {
    fprintf(stderr, "%f != %f\n", left, right);
    exit(EXIT_FAILURE);
  }

  char* file_name = "app/datasets/matmul/dataset.txt";

  double** m_a = mm_init(size);
  double** m_b = mm_init(size);
  double** m_c = mm_init(size);

  get_data(file_name, m_a, size);

  m_b = m_a;

  m_c = mm_mul(size, m_a, m_b);

  // Prints resulting matrix
  /*for (size_t i = 0; i < size; ++i) {
    for (size_t j = 0; j < size; ++j) {
      printf("%lf ", m_c[i][j]);
    }
    printf("\n");
  }*/

  return 0;

}