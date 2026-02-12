from dataclasses import dataclass


@dataclass
class AppConfig:
    debug: bool = False
    max_items: int = 10000
    default_page_size: int = 20
    max_page_size: int = 100
    cache_ttl: int = 300

    @classmethod
    def from_env(cls) -> "AppConfig":
        return cls()


DEFAULT_CONFIG = AppConfig()


def get_config() -> AppConfig:
    return DEFAULT_CONFIG
