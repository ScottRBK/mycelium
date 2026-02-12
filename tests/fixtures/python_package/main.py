from app.services.user_service import UserService
from app.models import User


def main():
    svc = UserService()
    user = svc.create_user("alice", "alice@example.com")
    print(user.name)


if __name__ == "__main__":
    main()
