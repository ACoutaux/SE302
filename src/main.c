#include <stdio.h>
#include <zephyr/kernel.h>

int main() {
    printf("Hello, world\n");
    k_thread_suspend(k_current_get());
    return 0;
}