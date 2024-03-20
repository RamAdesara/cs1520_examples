def squares():
    n = 2
    while True:
        yield n
        n = n * 2

for s in squares():
    print(f"{s} squared is ", end="")
    if s > 500:
        break