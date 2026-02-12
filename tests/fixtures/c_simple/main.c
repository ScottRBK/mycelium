#include "service.h"
#include "repository.h"
#include "types.h"
#include <stdio.h>
#include <stdlib.h>

struct Handler {
    int id;
    Config config;
};

void handle_request(struct Handler* h, int item_id) {
    const char* result = get_item(item_id);
    if (result[0] == '\0') {
        log_message(LOG_WARN, "Item not found");
        return;
    }
    log_message(LOG_INFO, result);
}

void handle_create(struct Handler* h, const char* name) {
    int id = create_item(name);
    if (id < 0) {
        log_message(LOG_ERROR, "Failed to create item");
        return;
    }
    log_message(LOG_INFO, "Item created");
}

void handle_delete(struct Handler* h, int item_id) {
    delete_item(item_id);
    log_message(LOG_INFO, "Item deleted");
}

void handle_list(struct Handler* h) {
    Item items[50];
    int count = list_items(items, 50);
    for (int i = 0; i < count; i++) {
        log_message(LOG_INFO, items[i].name);
    }
}

int main() {
    Config cfg = default_config();
    struct Handler h = {0, cfg};
    handle_create(&h, "test-item");
    handle_request(&h, 1);
    handle_list(&h);
    handle_delete(&h, 1);
    return 0;
}
