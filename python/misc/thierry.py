class Game:
    def __init__(self, name, version):
        self.name = name
        self.version = version

    def __add__(self, other):
        return Game(self.name + other.name, 1)

    def update(self):
        self.version += 1

    def __str__(self):
        return f"{self.name} {self.version}"


gta = Game("GTA", 4)
fifa10 = Game("FIFA", 10)
new = fifa10 + gta
print(gta)
gta.update()
print(gta)
