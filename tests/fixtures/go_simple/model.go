package model

type Item struct {
	Id       int
	Name     string
	Category string
	Price    float64
	Active   bool
}

type ItemFilter struct {
	Category string
	Active   *bool
	MinPrice float64
	MaxPrice float64
}

type PaginatedResult struct {
	Items      []Item
	Total      int
	Page       int
	PageSize   int
}

func NewItem(id int, name string) Item {
	return Item{
		Id:     id,
		Name:   name,
		Active: true,
	}
}
