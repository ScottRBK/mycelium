package service

import "myapp/model"

type DataService struct {
	store map[int]*model.Item
	counter int
}

func NewDataService() *DataService {
	return &DataService{
		store: make(map[int]*model.Item),
	}
}

func (s *DataService) Create(name string) *model.Item {
	s.counter++
	item := model.NewItem(s.counter, name)
	s.store[s.counter] = item
	return item
}

func (s *DataService) Get(id int) *model.Item {
	return s.store[id]
}
