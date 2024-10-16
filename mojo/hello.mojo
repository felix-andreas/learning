from collections import List

def main():
    l = List[Int](1, 2, 3)
    print("before:", end=" ")
    print_list(l)

    change_list(l)

    print("after:", end=" ")
    print_list(l)

fn print_list(list: List[Int]):
  print(end="[")
  for e in list:
    print(e[], end=", ")
  print(end="]\n")

def change_list(l: List[Int]):
    l[0] = 99
    print("inside:", end=" ")
    print_list(l)
