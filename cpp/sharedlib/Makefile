libhello.so: hello.cpp
	g++ -fPIC -shared -o $@ $^ 

test_libhello: test_libhello.cpp libhello.so
	g++ -o $@ $< -L. -lhello
