#include "repository.h"
#include <stdlib.h>
#include <string.h>

Repository* repo_create(int capacity) {
    Repository* repo = malloc(sizeof(Repository));
    if (!repo) return NULL;
    repo->items = malloc(sizeof(Item) * capacity);
    if (!repo->items) {
        free(repo);
        return NULL;
    }
    repo->capacity = capacity;
    repo->size = 0;
    return repo;
}

void repo_destroy(Repository* repo) {
    if (repo) {
        free(repo->items);
        free(repo);
    }
}

int repo_add(Repository* repo, const Item* item) {
    if (repo->size >= repo->capacity) return -1;
    repo->items[repo->size] = *item;
    repo->size++;
    return 0;
}

Item* repo_find(Repository* repo, int id) {
    for (int i = 0; i < repo->size; i++) {
        if (repo->items[i].id == id) {
            return &repo->items[i];
        }
    }
    return NULL;
}

int repo_remove(Repository* repo, int id) {
    for (int i = 0; i < repo->size; i++) {
        if (repo->items[i].id == id) {
            repo->items[i] = repo->items[repo->size - 1];
            repo->size--;
            return 1;
        }
    }
    return 0;
}

int repo_count(Repository* repo) {
    return repo->size;
}
