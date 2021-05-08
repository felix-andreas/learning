class Foo:
  num = 1
  pass

class Bar:
  num = 2
  pass

f1,f2= Foo(), Foo()
print(id(f1), id(f2))
f2.__class__ = Bar
print(id(f1),'\n', id(f2))
print( type(f1), type(f2), f1.num, f2.num )