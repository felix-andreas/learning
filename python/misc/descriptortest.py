import random


class triggered_property(property):
    def __init__(self, fget=None, fset=None, depends_on=None, fdel=None, doc=None):
        """
        A descriptor similar to property. The data is only computed if on of the
        objects it depends on changes.
        Args:
            fget: getter method
            fset: setter method
            depends_on: list of objects the object depends on
        """
        super().__init__(fget, fset, fdel, doc)
        self.data = None
        self._changed = True
        self.dependents = set()
        self.depends_on = depends_on or []
        for x in self.depends_on:
            x.dependents.add(self)

    @property
    def changed(self):
        return self._changed

    @changed.setter
    def changed(self, value):
        self._changed = value
        if value:
            for x in self.dependents:
                x.has_changed = value

    def __get__(self, instance, owner=None):
        if self.changed:
            self.data = super().__get__(instance, owner)
            print("hi")
            self.changed = False
        return self.data

    def __set__(self, obj, value):
        super().__set__(obj, value)
        self.data = obj.trash




class Line:
    def __init__(self, tree):
        self.tree = tree

    def get_length(self):
        return sum(self.tree)

    length = triggered_property(get_length)


tree = [1, 2, 3]
tree2 = [1, 2, 3, 4, 5, 6]

line = Line(tree)
line2 = Line(tree2)
print(line.length)
print(line2.length)
