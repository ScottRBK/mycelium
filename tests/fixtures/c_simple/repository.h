#ifndef REPOSITORY_H
#define REPOSITORY_H

#include "service.h"

typedef struct {
    Item* items;
    int capacity;
    int size;
} Repository;

Repository* repo_create(int capacity);
void repo_destroy(Repository* repo);
int repo_add(Repository* repo, const Item* item);
Item* repo_find(Repository* repo, int id);
int repo_remove(Repository* repo, int id);
int repo_count(Repository* repo);

#endif
