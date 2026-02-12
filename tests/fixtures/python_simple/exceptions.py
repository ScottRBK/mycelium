class AppError(Exception):
    def __init__(self, message: str, code: str = "UNKNOWN"):
        self.message = message
        self.code = code
        super().__init__(message)


class NotFoundError(AppError):
    def __init__(self, resource: str, resource_id: int):
        super().__init__(
            f"{resource} with id {resource_id} not found",
            code="NOT_FOUND",
        )
        self.resource = resource
        self.resource_id = resource_id


class ConflictError(AppError):
    def __init__(self, message: str):
        super().__init__(message, code="CONFLICT")


class ForbiddenError(AppError):
    def __init__(self, action: str):
        super().__init__(
            f"Not authorized to perform: {action}",
            code="FORBIDDEN",
        )
