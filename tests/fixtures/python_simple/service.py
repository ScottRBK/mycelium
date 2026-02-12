from typing import Optional
from models import Item, CreateItemRequest
from repository import ItemRepository


class DataService:
    def __init__(self):
        self._repository = ItemRepository()
        self._counter = 0

    def get_item(self, item_id: int) -> Optional[Item]:
        return self._repository.find_by_id(item_id)

    def create_item(self, request: CreateItemRequest) -> Item:
        self._counter += 1
        item = Item(
            id=self._counter,
            name=request.name,
            category=request.category,
            price=request.price,
            active=True,
        )
        self._repository.save(item)
        return item

    def delete_item(self, item_id: int) -> bool:
        existing = self._repository.find_by_id(item_id)
        if existing is None:
            return False
        self._repository.delete(item_id)
        return True

    def list_items(self, category: str = None, active_only: bool = True) -> list:
        items = self._repository.find_all()
        if category:
            items = [i for i in items if i.category == category]
        if active_only:
            items = [i for i in items if i.active]
        return items

    def update_item(self, item_id: int, updates: dict) -> Item:
        item = self._repository.find_by_id(item_id)
        if item is None:
            raise ValueError(f"Item {item_id} not found")
        for key, value in updates.items():
            if hasattr(item, key):
                setattr(item, key, value)
        self._repository.save(item)
        return item

    def search(self, query: str) -> list:
        items = self._repository.find_all()
        query_lower = query.lower()
        return [i for i in items if query_lower in i.name.lower()]
