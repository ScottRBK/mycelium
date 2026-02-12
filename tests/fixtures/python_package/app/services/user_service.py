import os

from pydantic import BaseModel
from app.models.user import User
from app.utils.helpers import format_name
from ..models.item import Item


class UserService:
    def __init__(self):
        self._users = []

    def create_user(self, name: str, email: str) -> User:
        formatted = format_name(name)
        user = User(formatted, email)
        self._users.append(user)
        return user

    def get_user(self, email: str):
        for user in self._users:
            if user.email == email:
                return user
        return None
