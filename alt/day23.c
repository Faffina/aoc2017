#include <stdio.h>
#include <stdint.h>

#define set(a, b) a = b
#define sub(a, b) a -= b
#define mul(a, b) a *=b
#define jnz(a, b) if (a != 0) goto b

int main() {
    int64_t count;
    int64_t a, b, c, d, e, f, g, h;
    a = 1;

    set (b, 99);
    set (c, b);
    jnz (a, a1);
    jnz (1, a2);
a1: mul (b, 100);
    sub (b, -100000);
    set (c, b);
    sub (c, -17000);
a2: set (f, 1);
    set (d, 2);
a5: set (e, 2);
a4: set (g, d);
    mul (g, e);
    sub (g, b);
    jnz (g, a3);
    set (f, 0);
a3: sub (e, -1);
    set (g, e);
    sub (g, b);
    jnz (g, a4);
    sub (d, -1);
    set (g, d);
    sub (g, b);
    jnz (g, a5);
    jnz (f, a6);
    sub (h, -1);
a6: set (g, b);
    sub (g, c);
    jnz (g, a7);
    jnz (1, end);
a7: sub (b, -17);
    jnz (1, a2);
end:printf("%ld \n", h);
}
