#%% Simple example
class Ten:
    def __get__(self, obj, objtype=None):
        return 10


class A:
    x = 5
    y = Ten()


a = A()
print(a.x, a.y)
print(A.__dict__)

#%% Dynamic lookups
from pathlib import Path


class DirectorySize:
    def __get__(self, obj, cls=None):
        return len(list(obj.path.iterdir()))


class Directory:
    size = DirectorySize()

    def __init__(self, path: Path) -> None:
        self.path = path


this = Directory(Path(__file__).parent)
parent = Directory(Path(__file__).parent.parent)
print(f". has size of {this.size}")
print(f".. has size of {parent.size}")

# %% Managed Attributes / Customized names

import logging
from rich.logging import RichHandler

logging.basicConfig(
    level="NOTSET", format="%(message)s", datefmt="[%X]", handlers=[RichHandler()]
)


class LoggedAgeAccess:
    logger = logging.getLogger("rich")

    def __init__(self):
        self.values = {}

    def __set_name__(self, _, name):
        self.name = name

    def __get__(self, obj, cls=None):
        value = self.values[obj]
        self.logger.info(f"Acessing {self.name}: {value}")
        return value

    def __set__(self, obj, value):
        self.values[obj] = value
        self.logger.info(f"Updating {self.name}: {value}")


class Person:
    age = LoggedAgeAccess()
    name = LoggedAgeAccess()

    def __init__(self, name, age):
        self.name = name
        self.age = age

    def birthday(self):
        self.age += 1


maría = Person("María", 23)
felix = Person("Felix", 26)
maría.birthday()

for person in maría, felix:
    print(person.name, person.age)

# %%
