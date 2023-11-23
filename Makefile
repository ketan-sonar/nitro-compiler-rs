run: link
	./output

link: assemble
	ld -o output output.o \
		-lSystem \
		-syslibroot `xcrun -sdk macosx --show-sdk-path` \
		-e _start \
		-arch arm64

assemble: build
	as -o output.o output.s

build:
	cargo r -- test.nt output.s

clean:
	rm output.*
