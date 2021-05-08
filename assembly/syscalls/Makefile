all: write getcwd

%: %.s
	mkdir -p build
	as -msyntax=intel -mnaked-reg -o build/$@.o $^
	ld build/$@.o -o build/$@


write2: write2.s
	mkdir -p build
	as -msyntax=intel -mnaked-reg -o build/$@.o $^
	ld build/$@.o -o build/$@

clean:
	rm -rf build
