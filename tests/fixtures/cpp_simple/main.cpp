#include "service.hpp"
#include "repository.hpp"
#include "models.hpp"
#include <iostream>

void printUsage() {
    std::cout << "Usage: app [create|get|list|delete] [args...]" << std::endl;
}

int runApp(int argc, const char* argv[]) {
    DataService service;
    ItemRepository repo;

    int id = service.createItem("sample");
    repo.save({id, "sample", "default", true});

    auto items = service.listItems();
    for (const auto& item : items) {
        std::cout << item.id << ": " << item.name << std::endl;
    }

    return 0;
}
