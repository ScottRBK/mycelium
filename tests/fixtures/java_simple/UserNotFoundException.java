package com.example.exceptions;

public class UserNotFoundException extends RuntimeException {
    private final int userId;

    public UserNotFoundException(int userId) {
        super("User not found with id: " + userId);
        this.userId = userId;
    }

    public int getUserId() {
        return userId;
    }
}
