%.o: %.s
	nasm -f macho32 $< -o $@

%: %.o
	ld -macosx_version_min 10.7.0 -lSystem -o $@ $<
