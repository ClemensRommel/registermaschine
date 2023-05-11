#include <stdio.h>

int main() {
    char* data = "Hello world!";
     
    char* pointer = data;
    int length = 12;

    while(pointer - data < length) {
        printf("%c", *pointer);
        pointer++;
    }

    printf("\n");
}