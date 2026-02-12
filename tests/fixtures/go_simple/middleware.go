package middleware

import "fmt"

type Logger struct {
	prefix string
}

func NewLogger(prefix string) *Logger {
	return &Logger{prefix: prefix}
}

func (l *Logger) Info(message string, id int) {
	fmt.Printf("[INFO] %s: %s (id=%d)\n", l.prefix, message, id)
}

func (l *Logger) Warn(message string, id int) {
	fmt.Printf("[WARN] %s: %s (id=%d)\n", l.prefix, message, id)
}

func (l *Logger) Error(message string, err error) {
	fmt.Printf("[ERROR] %s: %s - %v\n", l.prefix, message, err)
}

type RequestTimer struct {
	enabled bool
}

func NewRequestTimer() *RequestTimer {
	return &RequestTimer{enabled: true}
}

func (t *RequestTimer) IsEnabled() bool {
	return t.enabled
}
