#include <stdio.h>
#include <stdlib.h>

typedef int PrimeNumberItem_t;

int main(int argc, char **argv)
{

    PrimeNumberItem_t prime_number_count = 10;

    PrimeNumberItem_t curr_number = 0;
    while (prime_number_count > 0) {
        curr_number++;

        PrimeNumberItem_t j = 0;
        for (
            PrimeNumberItem_t i = 1;
            i <= curr_number;
            i++
            ) {
            if (curr_number % i == 0) {
                j++;
            }
        }

        if (j == 2) {
            prime_number_count--;
        }
    }

    printf("The latest prime number:\n");
    printf("%d\n", curr_number);
    return 0;
}