from typing import Protocol

class Greeter(Protocol) :
    def greet(self, name: str) -> str:
        """Greet somebody."""


class American:
    def greet(self, name: str) -> str:
        return f"Hello {name}"


class German:
    def greet(self, name: str) -> str:
        return f"Hallo {name}"


def main():
    people: list[Greeter] = [German(), American()]
    for person in people:
        person.greet("Foo")


if __name__ == "__main__":
    main()
