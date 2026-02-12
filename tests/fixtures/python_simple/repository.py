from typing import Optional
from models import Item


class ItemRepository:
    def __init__(self):
        self._store: dict[int, Item] = {}

    def find_by_id(self, item_id: int) -> Optional[Item]:
        return self._store.get(item_id)

    def find_all(self) -> list[Item]:
        return list(self._store.values())

    def save(self, item: Item) -> None:
        self._store[item.id] = item

    def delete(self, item_id: int) -> bool:
        if item_id in self._store:
            del self._store[item_id]
            return True
        return False

    def find_by_category(self, category: str) -> list[Item]:
        return [item for item in self._store.values() if item.category == category]

    def count(self) -> int:
        return len(self._store)

    def exists(self, item_id: int) -> bool:
        return item_id in self._store

    def find_by_name(self, name: str) -> Optional[Item]:
        for item in self._store.values():
            if item.name == name:
                return item
        return None

    def clear(self) -> None:
        self._store.clear()
