from dataclasses import dataclass, field
from typing import Optional
from enum import Enum


class ItemCategory(Enum):
    ELECTRONICS = "electronics"
    CLOTHING = "clothing"
    FOOD = "food"
    BOOKS = "books"
    OTHER = "other"


@dataclass
class Item:
    id: int
    name: str
    category: str = "other"
    price: float = 0.0
    active: bool = True
    tags: list = field(default_factory=list)

    def to_dict(self) -> dict:
        return {
            "id": self.id,
            "name": self.name,
            "category": self.category,
            "price": self.price,
            "active": self.active,
            "tags": self.tags,
        }


@dataclass
class CreateItemRequest:
    name: str
    category: str = "other"
    price: float = 0.0
    tags: list = field(default_factory=list)


@dataclass
class PaginatedResult:
    items: list
    total: int
    page: int
    page_size: int

    @property
    def has_next(self) -> bool:
        return self.page * self.page_size < self.total
