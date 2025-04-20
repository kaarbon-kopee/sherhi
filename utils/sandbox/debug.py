def fn(count: int) -> None:
    for i in range(count):
        print(i)
        if i == 5:
            break

if __name__ == "__main__":
    fn(10)