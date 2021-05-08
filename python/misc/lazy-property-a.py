class Dis:
    def __init__(self, func):
        self.func = func
        self.value = None

    def __get__(self, instance, owner):
        if self.value is None:
            self.value = self.func(instance)
        return self.value


class Magnet:
    def __init__(self, length):
        self.length = length


class Cell:
    def __init__(self, magnets):
        self.magnets = magnets
        self._length = property(self._length)
        # self.length = 10

    @Dis
    def _length(self):
        return sum([magnet.length for magnet in self.magnets])


if __name__ == "__main__":
    a = Magnet(2)
    b = Magnet(3)
    c = Cell([a, b])
    print(c.length)
