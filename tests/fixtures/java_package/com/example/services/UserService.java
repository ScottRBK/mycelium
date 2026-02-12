package com.example.services;

import com.example.models.User;
import java.util.List;
import java.util.ArrayList;

public class UserService {
    private List<User> users = new ArrayList<>();

    public User findById(String id) {
        return users.stream()
            .filter(u -> u.getId().equals(id))
            .findFirst()
            .orElse(null);
    }

    public void addUser(User user) {
        users.add(user);
    }

    public List<User> listAll() {
        return users;
    }
}
