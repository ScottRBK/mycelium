package service

import "myapp/model"

type DataService struct {
	store map[int]model.Item
	count int
}

type ItemRecord struct {
	Id   int
	Name string
}

func NewDataService() *DataService {
	return &DataService{
		store: make(map[int]model.Item),
	}
}

func (s *DataService) GetItem(id int) string {
	item, ok := s.store[id]
	if !ok {
		return ""
	}
	return item.Name
}

func (s *DataService) CreateItem(name string) int {
	s.count++
	s.store[s.count] = model.Item{
		Id:     s.count,
		Name:   name,
		Active: true,
	}
	return s.count
}

func (s *DataService) DeleteItem(id int) bool {
	if _, ok := s.store[id]; !ok {
		return false
	}
	delete(s.store, id)
	return true
}

func (s *DataService) ListItems() []ItemRecord {
	records := make([]ItemRecord, 0, len(s.store))
	for _, item := range s.store {
		records = append(records, ItemRecord{Id: item.Id, Name: item.Name})
	}
	return records
}

func (s *DataService) UpdateItem(id int, name string) bool {
	item, ok := s.store[id]
	if !ok {
		return false
	}
	item.Name = name
	s.store[id] = item
	return true
}
