#pragma once
#include "seraplot.g.h"
#include <fstream>
#include <stdexcept>
#include <string>
#include <utility>

namespace seraplot {

class Chart {
public:
    explicit Chart(std::string html) : html_(std::move(html)), handle_(nullptr) {}

    Chart(const Chart&) = delete;
    Chart& operator=(const Chart&) = delete;

    Chart(Chart&& other) noexcept : html_(std::move(other.html_)), handle_(other.handle_) {
        other.handle_ = nullptr;
    }

    Chart& operator=(Chart&& other) noexcept {
        if (this != &other) {
            release();
            html_ = std::move(other.html_);
            handle_ = other.handle_;
            other.handle_ = nullptr;
        }
        return *this;
    }

    ~Chart() { release(); }

    const std::string& html() {
        if (handle_) {
            char* r = sera_chart_html(handle_);
            html_.assign(r ? r : "");
            seraplot_free(r);
        }
        return html_;
    }

    void save(const std::string& path) { std::ofstream(path, std::ios::binary) << html(); }

    Chart& call(const std::string& name, const std::string& argsJson = "{}") {
        if (!handle_) handle_ = sera_chart_from_html(html_.c_str());
        void* updated = sera_chart_call(handle_, name.c_str(), argsJson.c_str());
        if (!updated) throw std::runtime_error("Methode chart inconnue : " + name);
        sera_chart_free(handle_);
        handle_ = updated;
        html_.clear();
        return *this;
    }

    std::string toSvg() {
        bool owned = handle_ == nullptr;
        void* h = owned ? sera_chart_from_html(html_.c_str()) : handle_;
        char* r = sera_chart_to_svg(h);
        std::string out(r ? r : "");
        seraplot_free(r);
        if (owned) sera_chart_free(h);
        return out;
    }

private:
    std::string html_;
    void* handle_;

    void release() {
        if (handle_) {
            sera_chart_free(handle_);
            handle_ = nullptr;
        }
    }
};

}
