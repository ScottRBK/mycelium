package com.example.repositories;

import com.example.models.User;
import java.util.List;

public interface UserRepository {
    User findById(int id);
    List<User> findAll();
    void save(User user);
    void delete(int id);
    int count();
}
