package com.example.models;

public class User {
    private int id;
    private String name;
    private String email;
    private boolean active;

    public User(int id, String name, String email) {
        this.id = id;
        this.name = name;
        this.email = email;
        this.active = true;
    }

    public int getId() { return id; }
    public void setId(int id) { this.id = id; }

    public String getName() { return name; }
    public void setName(String name) { this.name = name; }

    public String getEmail() { return email; }
    public void setEmail(String email) { this.email = email; }

    public boolean isActive() { return active; }
    public void setActive(boolean active) { this.active = active; }

    @Override
    public String toString() {
        return "User{id=" + id + ", name='" + name + "'}";
    }
}
