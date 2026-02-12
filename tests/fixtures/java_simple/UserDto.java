package com.example.models;

public class UserDto {
    private final int id;
    private final String displayName;
    private final String email;
    private final boolean active;

    public UserDto(int id, String displayName, String email, boolean active) {
        this.id = id;
        this.displayName = displayName;
        this.email = email;
        this.active = active;
    }

    public int getId() { return id; }
    public String getDisplayName() { return displayName; }
    public String getEmail() { return email; }
    public boolean isActive() { return active; }
}
