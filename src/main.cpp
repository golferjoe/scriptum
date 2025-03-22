#include <format>
#include <fstream>
#include <print>
#include <string>

std::string trim(const std::string &str) {
    auto start = str.find_first_not_of(" \t\n");
    auto end = str.find_last_not_of(" \t\n");

    if (start == std::string::npos) {
        return "";
    }

    return str.substr(start, end - start + 1);
}

std::string parseLine(const std::string &line) {
    // TODO: try to find a better way of implementing parsing instead of using 10000 if statements

    const auto &trimmed = trim(line);

    // extract tag
    if (trimmed.starts_with("[h:")) {
        // find tag closing bracket position
        const auto closingBracket = trimmed.find(']');
        if (closingBracket == std::string::npos) {
            return "";
        }

        // extract tag type and text
        const std::string tag = trimmed.substr(3, closingBracket - 3);
        const std::string text = trim(trimmed.substr(closingBracket + 1));

        // parse into html heading
        if (tag == "big") {
            return std::format("<h1>{}<h2>", text);
        } else if (tag == "med") {
            return std::format("<h2>{}<h2>", text);
        } else if (tag == "sma") {
            return std::format("<h3>{}<h3>", text);
        }
    }

    return "";
}

int main(int argc, char *argv[]) {
    // check if user provided enough arguments
    if (argc < 3) {
        std::print("Not enough arguments provided!\nSyntax: anl <source> <output>\n");
        return 1;
    }

    // extract args
    const auto sourceName = argv[1];
    const auto outputName = argv[2];

    // try to open input file
    std::ifstream source(sourceName);
    if (!source.is_open()) {
        std::print("Failed to open source file! Does the file exists?\n");
        return 1;
    }

    // read file line by line
    std::string line;
    while (std::getline(source, line)) {
        const auto &html = parseLine(line);

        if (!html.empty()) {
            std::print("{}\n", html);
        }
    }

    return 0;
}
