all: hello

hello.o: hello.s
	nasm -f macho32 $< -o $@

hello: hello.o
	ld -macosx_version_min 10.7.0 -lSystem -o $@ $<
