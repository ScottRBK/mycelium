package main

import (
	"fmt"
	"myapp/service"
	"myapp/middleware"
)

func main() {
	svc := service.NewDataService()
	logger := middleware.NewLogger()

	logger.Info("Starting application")

	item := svc.Create("test-item")
	fmt.Println("Created:", item)
}
