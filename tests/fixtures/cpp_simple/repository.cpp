#include "repository.hpp"
#include <algorithm>

std::optional<ItemRecord> ItemRepository::findById(int id) const {
    int idx = findIndex(id);
    if (idx < 0) {
        return std::nullopt;
    }
    return items_[idx];
}

std::vector<ItemRecord> ItemRepository::findAll() const {
    std::vector<ItemRecord> active;
    for (const auto& item : items_) {
        if (item.active) {
            active.push_back(item);
        }
    }
    return active;
}

void ItemRepository::save(const ItemRecord& item) {
    int idx = findIndex(item.id);
    if (idx >= 0) {
        items_[idx] = item;
    } else {
        items_.push_back(item);
    }
}

bool ItemRepository::remove(int id) {
    int idx = findIndex(id);
    if (idx < 0) return false;
    items_[idx].active = false;
    return true;
}

int ItemRepository::count() const {
    int active = 0;
    for (const auto& item : items_) {
        if (item.active) active++;
    }
    return active;
}

bool ItemRepository::exists(int id) const {
    return findIndex(id) >= 0;
}

int ItemRepository::findIndex(int id) const {
    for (size_t i = 0; i < items_.size(); i++) {
        if (items_[i].id == id) {
            return static_cast<int>(i);
        }
    }
    return -1;
}
