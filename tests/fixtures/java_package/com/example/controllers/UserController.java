package com.example.controllers;

import com.example.services.UserService;
import com.example.models.User;
import java.util.List;

public class UserController {
    private final UserService service;

    public UserController(UserService service) {
        this.service = service;
    }

    public User getUser(String id) {
        return service.findById(id);
    }

    public void createUser(String id, String name, String email) {
        User user = new User(id, name, email);
        service.addUser(user);
    }

    public List<User> listUsers() {
        return service.listAll();
    }
}
