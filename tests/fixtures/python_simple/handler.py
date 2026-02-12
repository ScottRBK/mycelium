from service import DataService
from models import Item, CreateItemRequest
from validators import ItemValidator


class RequestHandler:
    def __init__(self):
        self.service = DataService()
        self.validator = ItemValidator()

    def handle_get(self, item_id: int) -> dict:
        item = self.service.get_item(item_id)
        if item is None:
            raise ItemNotFoundError(f"Item {item_id} not found")
        return item.to_dict()

    def handle_create(self, data: dict) -> dict:
        validated = self._validate(data)
        request = CreateItemRequest(**validated)
        item = self.service.create_item(request)
        return item.to_dict()

    def handle_delete(self, item_id: int) -> bool:
        existing = self.service.get_item(item_id)
        if existing is None:
            raise ItemNotFoundError(f"Item {item_id} not found")
        return self.service.delete_item(item_id)

    def handle_list(self, category: str = None, limit: int = 50) -> list:
        items = self.service.list_items(category=category)
        return [item.to_dict() for item in items[:limit]]

    def handle_update(self, item_id: int, data: dict) -> dict:
        validated = self._validate(data)
        item = self.service.update_item(item_id, validated)
        return item.to_dict()

    def _validate(self, data: dict) -> dict:
        errors = self.validator.validate(data)
        if errors:
            raise ValidationError(errors)
        return data


class ItemNotFoundError(Exception):
    pass


class ValidationError(Exception):
    def __init__(self, errors: list):
        self.errors = errors
        super().__init__(f"Validation failed: {errors}")
