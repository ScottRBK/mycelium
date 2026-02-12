package com.example.repositories;

import com.example.models.User;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class InMemoryUserRepository implements UserRepository {
    private final Map<Integer, User> store = new HashMap<>();

    @Override
    public User findById(int id) {
        return store.get(id);
    }

    @Override
    public List<User> findAll() {
        return new ArrayList<>(store.values());
    }

    @Override
    public void save(User user) {
        store.put(user.getId(), user);
    }

    @Override
    public void delete(int id) {
        store.remove(id);
    }

    @Override
    public int count() {
        return store.size();
    }

    public boolean exists(int id) {
        return store.containsKey(id);
    }

    public void clear() {
        store.clear();
    }
}
