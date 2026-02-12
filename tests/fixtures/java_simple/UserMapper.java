package com.example.mappers;

import com.example.models.User;
import com.example.models.UserDto;

public class UserMapper {
    public UserDto toDto(User user) {
        return new UserDto(
            user.getId(),
            formatDisplayName(user.getName()),
            user.getEmail(),
            user.isActive()
        );
    }

    private String formatDisplayName(String name) {
        if (name == null || name.isEmpty()) {
            return "Unknown";
        }
        return name.substring(0, 1).toUpperCase() + name.substring(1);
    }
}
