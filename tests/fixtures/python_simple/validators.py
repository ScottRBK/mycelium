from typing import Optional


class ItemValidator:
    MAX_NAME_LENGTH = 200
    MIN_PRICE = 0.0
    MAX_PRICE = 99999.99
    VALID_CATEGORIES = {"electronics", "clothing", "food", "books", "other"}

    def validate(self, data: dict) -> list:
        errors = []

        if "name" not in data or not data["name"]:
            errors.append("Name is required")
        elif len(data["name"]) > self.MAX_NAME_LENGTH:
            errors.append(f"Name must be under {self.MAX_NAME_LENGTH} characters")

        if "price" in data:
            price = data["price"]
            if not isinstance(price, (int, float)):
                errors.append("Price must be a number")
            elif price < self.MIN_PRICE or price > self.MAX_PRICE:
                errors.append(f"Price must be between {self.MIN_PRICE} and {self.MAX_PRICE}")

        if "category" in data:
            if data["category"] not in self.VALID_CATEGORIES:
                errors.append(f"Invalid category: {data['category']}")

        return errors

    def validate_tags(self, tags: list) -> list:
        errors = []
        if len(tags) > 10:
            errors.append("Maximum 10 tags allowed")
        for tag in tags:
            if not isinstance(tag, str) or len(tag) > 50:
                errors.append(f"Invalid tag: {tag}")
        return errors
