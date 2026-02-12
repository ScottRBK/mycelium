#include "service.hpp"
#include "repository.hpp"
#include "models.hpp"
#include <iostream>
#include <vector>

namespace app {

class Handler {
public:
    Handler() : svc_(), repo_() {}

    std::string handleGet(int id) {
        auto result = svc_.getItem(id);
        if (result.empty()) {
            std::cerr << "Item not found: " << id << std::endl;
        }
        return result;
    }

    int handleCreate(const std::string& name) {
        if (name.empty()) {
            throw std::invalid_argument("Name cannot be empty");
        }
        return svc_.createItem(name);
    }

    bool handleDelete(int id) {
        return svc_.deleteItem(id);
    }

    std::vector<ItemRecord> handleList() {
        return svc_.listItems();
    }

    AppConfig getConfig() const {
        return config_;
    }

private:
    DataService svc_;
    ItemRepository repo_;
    AppConfig config_;
};

} // namespace app

int main() {
    app::Handler h;
    int id = h.handleCreate("test");
    std::cout << "Created: " << id << std::endl;
    std::cout << "Get: " << h.handleGet(id) << std::endl;
    auto items = h.handleList();
    std::cout << "Total: " << items.size() << std::endl;
    return 0;
}
