from .validators import validate_string


def format_name(name: str) -> str:
    validate_string(name)
    return name.strip().title()
