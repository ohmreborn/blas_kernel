CC = gcc
INCLUDES = /opt/homebrew/Cellar/openblas/0.3.29/include
INCLUDES_FLAGS = -I$(INCLUDES)
CFLAGS = -O3 -march=native -Wall $(INCLUDES_FLAGS)
LDFLAGS = -L/opt/homebrew/opt/openblas/lib

all: main.o
	$(CC) main.o $(LDFLAGS) -lopenblas -o main

main.o: main.c $(INCLUDES)/cblas.h
	$(CC) $(CFLAGS) -c main.c

clean:
	rm -f main main.o
