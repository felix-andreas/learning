# import functools
#
# def lazy_property(fn):
#     attr_name = '_lazy_' + fn.__name__
#
#     @property
#     @functools.wraps(fn)
#     def _lazy_property(self):
#         if getattr(self, attr_name) is None:
#             setattr(self, attr_name, fn(self))
#         return getattr(self, attr_name)
#
#     return _lazy_property
#
#
# class Magnet:
#     def __init__(self, length):
#         self.length = length
#
# class Cell:
#     def __init__(self, magnets):
#         self.magnets = magnets
#
#     @lazy_property
#     def length(self):
#         return sum([magnet.length for magnet in self.magnets])
#
# if __name__ == "__main__":
#     a = Magnet(2)
#     b = Magnet(3)
#     c = Cell([a, b])
#     print(c.length)