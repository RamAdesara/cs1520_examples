from typing import Any

class GenZChar:
    def __init__(self, char: str) -> None:
        if not isinstance(char, str) or len(char) != 1:
            raise ValueError("Can only create a GenZChar from a string of length 1!")

        self.char = char

    def __eq__(self, other: Any) -> str:
        if isinstance(other, GenZChar):
            other = other.char

        return "bet" if self.char == other else "bruh."

    def __hash__(self) -> int:
        return hash(("GenZChar", self.char))

    def __str__(self) -> str:
        return self.char

    def __repr__(self) -> str:
        return f"<GenZChar {self.char}>"

def main():
    a = GenZChar("a")
    print(a)
    print(repr(a))
    print()
    
    b = GenZChar("b")
    other_a = GenZChar("a")
    print(a == other_a)
    print(a == b)
    print(a == "a")
    print()

    a_set = {a, b, other_a}
    print(a_set)
    print(a in a_set)
    print("a" in a_set)
    print(b in a_set)
    print()

    a_set.add("a")
    print(a_set)
    print(a in a_set)
    print("a" in a_set)
    print(b in a_set)
    print()

    c = GenZChar("c")
    print(c in a_set)
    print("c" in a_set)
    print()

    # errors:
    # two = GenZChar(2)
    # ab = GenZChar("ab")

if __name__ == "__main__":
    main()
