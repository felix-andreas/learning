pointer_to_array: pointer_to_array.c
	clang $^ -o $@

pass_a_struct: pass_a_struct.c
	clang -lm $^ -o $@

hello_world_object_file: hello_world_object_file.c
	clang -O3 -c $^

clean:
	rm pass_a_struct pointer_to_array