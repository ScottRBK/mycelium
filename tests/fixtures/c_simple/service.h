#ifndef SERVICE_H
#define SERVICE_H

typedef struct {
    int id;
    char name[256];
    int active;
} Item;

enum ItemStatus {
    ITEM_ACTIVE,
    ITEM_INACTIVE,
    ITEM_DELETED
};

const char* get_item(int id);
int create_item(const char* name);
void delete_item(int id);
int update_item(int id, const char* name);
int list_items(Item* buffer, int max_count);
int item_count(void);

#endif
