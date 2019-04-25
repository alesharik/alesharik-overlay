all:
	gcc -Wall -Wstrict-prototypes -Wmissing-prototypes -Wmissing-declarations -Wredundant-decls -lX11 -lXext -lXrender xwinwrap.c -o xwinwrap

install:
	cp xwinwrap /usr/bin

clean:
	-rm -rf xwinwrap

