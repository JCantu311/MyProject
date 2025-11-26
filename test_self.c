#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int explosion = 100;
int num = 0;
int num2 = 10;

int main()
{
    restartloop:;
    int num = 0;
    int num2 = 10;
    char loop1[4];
    for (int i = 0; i < 20; ++i)
    {
        printf("Hello, world! (C)\n");
        system("py test2.py");
    }
    system("pause");
    for (int i = explosion; i > explosion - 50; --i)
    {
        printf("Hello, Bitch! (C) ");
    }
    printf("\n");
    printf("Sorry, That was Mean.");
    printf("\n");
    system("pause");
    printf("\n");
    for (num; num < num2; ++num)
    {
        system("py test2.py");
    }
    printf("Do you have Rust installed? (y/n)");
    char yOrN;
    char charY = 'y';
    char charN = 'n';
    scanf(" %c", &yOrN);
    if (yOrN == charN) {
        printf("Install Rust before continuing\n");
        system("pause");
    } else if (yOrN == charY) {
        printf("Okay you can continue\n");
    }
    printf("(We're gonna try running 'rustc testRust.rs' to compile the testRust.rs Rust file but it probably won't work, apologies for the inevitable error)\n");
    system("rustc testRust.rs");
    system("testRust.exe");
    system("pause");
    printf("Enter 'loop' if you wish to restart\n");
    printf("Type anything else if you wish to quit: ");
    scanf("%s", loop1);
    if (strcmp(loop1, "loop") == 0)
    {
        printf("Looping");
        system("py looptest.py");
        goto restartloop;
    } else {
        system("pause");
    }
}
