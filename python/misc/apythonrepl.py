print("This is my Implementation of the Python REPL")
while True:
    print(">>> ",end="")
    ipt = input()
    try:
        exec(ipt)
    except Exception as e:
        print(e)
