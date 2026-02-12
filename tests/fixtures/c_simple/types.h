#ifndef TYPES_H
#define TYPES_H

typedef struct {
    int max_items;
    int debug_mode;
    int log_level;
} Config;

enum LogLevel {
    LOG_DEBUG,
    LOG_INFO,
    LOG_WARN,
    LOG_ERROR
};

Config default_config(void);
void log_message(int level, const char* message);

#endif
