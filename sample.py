import sys


class OperationsInterface(object):

    def sum(self, x: int, y: int) -> int:
        raise NotImplementedError

    def minus(self, x: int, y: int) -> int:
        raise NotImplementedError

    def mul(self, x: int, y: int) -> int:
        raise NotImplementedError


class MathOperationsImpl(OperationsInterface):

    def sum(self, x: int, y: int) -> int:
        return x + y

    def minus(self, x: int, y: int) -> int:
        return x - y

    def mul(self, x: int, y: int) -> int:
        return x * y


class DummyOperationsImpl(OperationsInterface):

    def sum(self, x: int, y: int) -> int:
        return 0

    def minus(self, x: int, y: int) -> int:
        return 0

    def mul(self, x: int, y: int) -> int:
        return 0


class OperationsFactory(object):

    def create(self, parameters: str):
        if parameters == "math":
            return MathOperationsImpl()
        elif parameters == "dummy":
            return DummyOperationsImpl()


def dhrystone(operations):
    pass


def main(args):
    assert len(args) > 1
    factory = OperationsFactory()
    operations = factory.create(args[1])

    for k in range(1000):
        dhrystone(operations)


if __name__ == '__main__':
    main(sys.argv)