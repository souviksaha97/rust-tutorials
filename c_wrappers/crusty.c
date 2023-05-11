#include <stdio.h>
#include "rusty.h"

int main(){
    int x=10;
    int y=20;
    int len = 3;
    uint32_t array[3] = {1, 2, 3};
    uint32_t arrax[3] = {10, 20, 30};
    printf("Hello world\r\n");
    printf("%d\r\n", adder(10, 20));
    int32_t sum = add_pointers(&x, &y);
    printf("The sum of %d and %d is %d\n", x, y, sum);
    uint32_t* result = sum_of_arrays(array, arrax, len);
    for (size_t i = 0; i < len; i++) {
        printf("%d + %d = %d\n", array[i], arrax[i], result[i]);
    }
    free(result);
    return 0;
}