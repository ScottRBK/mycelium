package com.example.services;

import com.example.models.User;
import com.example.repositories.UserRepository;
import com.example.repositories.InMemoryUserRepository;
import java.util.List;

public class UserService {
    private final UserRepository repository;

    public UserService() {
        this.repository = new InMemoryUserRepository();
    }

    public UserService(UserRepository repository) {
        this.repository = repository;
    }

    public User findById(int id) {
        return repository.findById(id);
    }

    public List<User> findAll() {
        return repository.findAll();
    }

    public User create(String name, String email) {
        validateInput(name);
        int id = generateId();
        User user = new User(id, name, email);
        repository.save(user);
        return user;
    }

    public void delete(int id) {
        User existing = repository.findById(id);
        if (existing == null) {
            throw new IllegalArgumentException("User not found: " + id);
        }
        repository.delete(id);
    }

    public User update(int id, String name, String email) {
        User existing = repository.findById(id);
        if (existing == null) {
            throw new IllegalArgumentException("User not found: " + id);
        }
        existing.setName(name);
        existing.setEmail(email);
        repository.save(existing);
        return existing;
    }

    private int generateId() {
        return repository.count() + 1;
    }

    private void validateInput(String name) {
        if (name == null || name.trim().isEmpty()) {
            throw new IllegalArgumentException("Name cannot be empty");
        }
    }
}

interface UserRepository {
    User findById(int id);
    List<User> findAll();
    void save(User user);
    void delete(int id);
    int count();
}
