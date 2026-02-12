#include "types.h"
#include <stdio.h>

Config default_config(void) {
    Config cfg;
    cfg.max_items = 100;
    cfg.debug_mode = 0;
    cfg.log_level = LOG_INFO;
    return cfg;
}

static const char* level_names[] = {"DEBUG", "INFO", "WARN", "ERROR"};

void log_message(int level, const char* message) {
    if (level >= LOG_INFO) {
        printf("[%s] %s\n", level_names[level], message);
    }
}
