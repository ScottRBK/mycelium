#pragma once

#include <string>
#include <vector>
#include <map>
#include <optional>

struct ItemRecord {
    int id;
    std::string name;
    std::string category;
    bool active;
};

enum class Status {
    Active,
    Inactive,
    Deleted
};

class DataService {
public:
    DataService();
    std::string getItem(int id) const;
    int createItem(const std::string& name);
    bool deleteItem(int id);
    std::vector<ItemRecord> listItems() const;
    bool updateItem(int id, const std::string& name);
    int count() const;

private:
    std::map<int, ItemRecord> store_;
    int counter_;

    bool isValidName(const std::string& name) const;
};
