package middleware

import "fmt"

type Logger struct {
	Level string
}

func NewLogger() *Logger {
	return &Logger{Level: "info"}
}

func (l *Logger) Info(msg string) {
	fmt.Println("[INFO]", msg)
}

func (l *Logger) Error(msg string) {
	fmt.Println("[ERROR]", msg)
}
