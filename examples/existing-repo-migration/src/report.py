def format_status(item: str, state: str) -> str:
    return f"{item.strip()}: {state.strip()}"


def main() -> None:
    print(format_status(" actuator ", " ready "))


if __name__ == "__main__":
    main()
