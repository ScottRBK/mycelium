#pragma once

#include "service.hpp"
#include <vector>
#include <optional>

class ItemRepository {
public:
    ItemRepository() = default;

    std::optional<ItemRecord> findById(int id) const;
    std::vector<ItemRecord> findAll() const;
    void save(const ItemRecord& item);
    bool remove(int id);
    int count() const;
    bool exists(int id) const;

private:
    std::vector<ItemRecord> items_;

    int findIndex(int id) const;
};
