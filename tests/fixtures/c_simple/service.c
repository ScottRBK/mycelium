#include "service.h"
#include <string.h>

#define MAX_ITEMS 100

static Item store[MAX_ITEMS];
static int count = 0;

const char* get_item(int id) {
    for (int i = 0; i < count; i++) {
        if (store[i].id == id && store[i].active) {
            return store[i].name;
        }
    }
    return "";
}

int create_item(const char* name) {
    if (count >= MAX_ITEMS) return -1;
    count++;
    store[count - 1].id = count;
    strncpy(store[count - 1].name, name, 255);
    store[count - 1].name[255] = '\0';
    store[count - 1].active = 1;
    return count;
}

void delete_item(int id) {
    for (int i = 0; i < count; i++) {
        if (store[i].id == id) {
            store[i].active = 0;
            return;
        }
    }
}

int update_item(int id, const char* name) {
    for (int i = 0; i < count; i++) {
        if (store[i].id == id && store[i].active) {
            strncpy(store[i].name, name, 255);
            store[i].name[255] = '\0';
            return 1;
        }
    }
    return 0;
}

int list_items(Item* buffer, int max_count) {
    int found = 0;
    for (int i = 0; i < count && found < max_count; i++) {
        if (store[i].active) {
            buffer[found] = store[i];
            found++;
        }
    }
    return found;
}

int item_count(void) {
    int active = 0;
    for (int i = 0; i < count; i++) {
        if (store[i].active) active++;
    }
    return active;
}
