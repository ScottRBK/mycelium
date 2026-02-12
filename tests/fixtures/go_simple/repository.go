package repository

import "myapp/model"

type Repository interface {
	FindById(id int) (*model.Item, bool)
	FindAll() []model.Item
	Save(item model.Item)
	Delete(id int) bool
	Count() int
}

type InMemoryRepository struct {
	items map[int]model.Item
}

func NewInMemoryRepository() *InMemoryRepository {
	return &InMemoryRepository{
		items: make(map[int]model.Item),
	}
}

func (r *InMemoryRepository) FindById(id int) (*model.Item, bool) {
	item, ok := r.items[id]
	if !ok {
		return nil, false
	}
	return &item, true
}

func (r *InMemoryRepository) FindAll() []model.Item {
	result := make([]model.Item, 0, len(r.items))
	for _, item := range r.items {
		result = append(result, item)
	}
	return result
}

func (r *InMemoryRepository) Save(item model.Item) {
	r.items[item.Id] = item
}

func (r *InMemoryRepository) Delete(id int) bool {
	if _, ok := r.items[id]; !ok {
		return false
	}
	delete(r.items, id)
	return true
}

func (r *InMemoryRepository) Count() int {
	return len(r.items)
}
