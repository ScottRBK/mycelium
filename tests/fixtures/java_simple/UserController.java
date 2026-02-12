package com.example.controllers;

import com.example.services.UserService;
import com.example.models.User;
import com.example.models.UserDto;
import com.example.mappers.UserMapper;
import com.example.exceptions.UserNotFoundException;
import java.util.List;
import java.util.stream.Collectors;

public class UserController {
    private final UserService userService;
    private final UserMapper mapper;

    public UserController() {
        this.userService = new UserService();
        this.mapper = new UserMapper();
    }

    public UserDto getUser(int id) {
        User user = userService.findById(id);
        if (user == null) {
            throw new UserNotFoundException(id);
        }
        return mapper.toDto(user);
    }

    public UserDto createUser(String name, String email) {
        User user = userService.create(name, email);
        logAction("Created user: " + name);
        return mapper.toDto(user);
    }

    public void deleteUser(int id) {
        User existing = userService.findById(id);
        if (existing == null) {
            throw new UserNotFoundException(id);
        }
        userService.delete(id);
        logAction("Deleted user: " + id);
    }

    public List<UserDto> listUsers() {
        return userService.findAll()
            .stream()
            .map(mapper::toDto)
            .collect(Collectors.toList());
    }

    private void logAction(String action) {
        System.out.println("[AUDIT] " + action);
    }

    private void handleError(Exception e) {
        System.err.println("[ERROR] " + e.getMessage());
    }
}
