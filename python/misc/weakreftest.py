class trigger:
    def __init__(self, dependet_of):
        self._changed = True
        self.dependents = set()
        self.dependent_of = set(dependet_of)
        for x in self.dependent_of:
            x.dependents.add(self)

    @property
    def changed(self):
        return self._changed

    @changed.setter
    def changed(self, value):
        self._changed = True
        if value:
            for x in self.dependents:
                x.has_changed = True