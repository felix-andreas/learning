class PropertyTrigger:
    def __init__(self, depends_on=None):
        """
        A container class for an attribute, which is only computed if one of the
        objects it depends on changes.
        Args:
            depends_on: list of objects the object depends on
        """
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

class Die:
    def __init__(self):
        self.prop = PropertyTrigger()


x = Die()