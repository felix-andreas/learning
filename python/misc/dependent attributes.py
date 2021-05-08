class _Center:
    def __get__(self, instance, owner):
        return instance.x + instance.width / 2.0, instance.y + instance.height / 2.0

    def __set__(self, instance, value):
        instance.x = value[0] - instance.width / 2.0
        instance.y = value[1] - instance.height / 2.0


class Rect:
    center = _Center()

    def __init__(self, x, y, width, height):
        self.x = x
        self.y = y
        self.__z = 10
        self.width = width
        self.height = height

    def __repr__(self):
        return "Rect({x}, {y}, {width}, {height})".format(**vars(self))



rec = Rect(0,0,1,1)
print(rec.center)
rec.center = (0,0)