class Human:
    def __init__(self):
        self._length = 2

    @property
    def length(self):
        return self._length

    @length.setter
    def length(self, value):
        if value > 2:
            self._length = 2
        else:
            self._length = value


felix = Human()
felix.length = 1.8
print(felix.length)
felix.length = 2.2
print(felix.length)
import weakref
a = weakref.proxy(felix)
print(a.length)
print(felix)