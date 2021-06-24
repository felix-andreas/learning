class Meta(type):
    def __new__(cls, name, bases, attrs):
        attrs = {
            key.capitalize() if not key.startswith("_") else key: value
            for key, value in attrs.items()
        }
        print(attrs.keys())
        return type(name, bases, attrs)


class Dog(metaclass=Meta):
    def bark(self):
        print("wuff wuff")


dog = Dog()
dog.Bark()  # language server shows error :(
