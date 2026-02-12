#pragma once

#include <string>

struct AppConfig {
    int maxItems = 1000;
    bool debugMode = false;
    std::string logLevel = "INFO";
};

struct ErrorResponse {
    int code;
    std::string message;

    ErrorResponse(int c, const std::string& m) : code(c), message(m) {}
};

struct PaginatedRequest {
    int page = 1;
    int pageSize = 20;
    std::string sortBy = "id";
    bool ascending = true;
};
