def validate_string(value: str) -> None:
    if not isinstance(value, str):
        raise TypeError("Expected string")
    if not value.strip():
        raise ValueError("Empty string")
