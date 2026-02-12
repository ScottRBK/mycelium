package model

type Item struct {
	ID   int
	Name string
}

func NewItem(id int, name string) *Item {
	return &Item{ID: id, Name: name}
}
