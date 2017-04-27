CC=gcc
CFLAGS=-std=c11 -Wall -pedantic -g3
DYLIB=-dynamiclib -undefined suppress -flat_namespace
PERF=-pg
PROD=-O2
NAME=fibonacci

dylib: fibonacci.o
	$(CC) $(DYLIB) $^ -o $(NAME).dylib

fibonacci.o: fibonacci.c

clean:
	$(RM) *.o $(NAME)
