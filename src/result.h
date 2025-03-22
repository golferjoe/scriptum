#pragma once

#include <string>

template <typename T>
struct Result {  // oh how much I love rust for having such types built-in :3
    T value;
    std::string error;
    bool is_ok;

    static Result<T> Ok(T v) {
        return { v, "", true };
    }
    static Result<T> Err(const std::string& msg) {
        return { {}, msg, false };
    }
};

extern Result<std::string> ok(const std::string& v) {
    return Result<std::string>::Ok(v);
}

extern Result<std::string> err(const std::string& v) {
    return Result<std::string>::Err(v);
}
