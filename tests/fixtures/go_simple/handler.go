package main

import (
	"fmt"
	"myapp/service"
	"myapp/middleware"
)

type Handler struct {
	svc *service.DataService
	log *middleware.Logger
}

func NewHandler() *Handler {
	return &Handler{
		svc: service.NewDataService(),
		log: middleware.NewLogger("handler"),
	}
}

func (h *Handler) HandleGet(id int) string {
	h.log.Info("Getting item", id)
	result := h.svc.GetItem(id)
	if result == "" {
		h.log.Warn("Item not found", id)
		return ""
	}
	return result
}

func (h *Handler) HandleCreate(name string) int {
	h.log.Info("Creating item", 0)
	id := h.svc.CreateItem(name)
	h.log.Info("Created item", id)
	return id
}

func (h *Handler) HandleDelete(id int) bool {
	h.log.Info("Deleting item", id)
	return h.svc.DeleteItem(id)
}

func (h *Handler) HandleList() []service.ItemRecord {
	return h.svc.ListItems()
}

func main() {
	h := NewHandler()
	id := h.HandleCreate("test-item")
	fmt.Println("Created:", id)
	fmt.Println("Get:", h.HandleGet(id))
	fmt.Println("All:", h.HandleList())
}
