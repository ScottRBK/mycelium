#include "service.hpp"

DataService::DataService() : counter_(0) {}

std::string DataService::getItem(int id) const {
    auto it = store_.find(id);
    if (it != store_.end() && it->second.active) {
        return it->second.name;
    }
    return "";
}

int DataService::createItem(const std::string& name) {
    if (!isValidName(name)) {
        return -1;
    }
    counter_++;
    ItemRecord record;
    record.id = counter_;
    record.name = name;
    record.category = "default";
    record.active = true;
    store_[counter_] = record;
    return counter_;
}

bool DataService::deleteItem(int id) {
    auto it = store_.find(id);
    if (it == store_.end()) {
        return false;
    }
    it->second.active = false;
    return true;
}

std::vector<ItemRecord> DataService::listItems() const {
    std::vector<ItemRecord> result;
    for (const auto& pair : store_) {
        if (pair.second.active) {
            result.push_back(pair.second);
        }
    }
    return result;
}

bool DataService::updateItem(int id, const std::string& name) {
    auto it = store_.find(id);
    if (it == store_.end() || !it->second.active) {
        return false;
    }
    it->second.name = name;
    return true;
}

int DataService::count() const {
    int active = 0;
    for (const auto& pair : store_) {
        if (pair.second.active) active++;
    }
    return active;
}

bool DataService::isValidName(const std::string& name) const {
    return !name.empty() && name.length() <= 256;
}
