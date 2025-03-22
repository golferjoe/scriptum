#include <format>
#include <fstream>
#include <print>
#include <string>

#include "base64.h"

// TODO: modularize code
// TODO: make image file path relative to source file

std::string trim(const std::string &str) {
    auto start = str.find_first_not_of(" \t\n");
    auto end = str.find_last_not_of(" \t\n");

    if (start == std::string::npos) {
        return "";
    }

    return str.substr(start, end - start + 1);
}

std::string getFileExt(const std::string &fileName) {
    auto dotPos = fileName.find_last_of('.');

    if (dotPos == std::string::npos) {
        return "";
    }

    return fileName.substr(dotPos + 1);
}

std::string getMimeType(const std::string &ext) {
    if (ext == "png")
        return "image/png";
    else if (ext == "jpg" || ext == "jpeg")
        return "image/jpeg";
    else if (ext == "gif")
        return "image/gif";
    else
        return "application/octet-stream";
}

std::string parseLine(const std::string &line) {
    const auto &trimmed = trim(line);
    const auto closingBracket = trimmed.find(']');

    // extract tag
    if (trimmed.starts_with("[h:")) {
        // find tag closing bracket position
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
    } else if (trimmed.starts_with("[b]")) {
        if (closingBracket == std::string::npos) {
            return "";
        }
        const std::string text = trim(trimmed.substr(closingBracket + 1));
        return std::format("<b>{}</b>", text);
    } else if (trimmed.starts_with("[img:")) {
        // extract image path
        const auto colonPos = trimmed.find(':');
        const std::string imageName = trimmed.substr(colonPos + 1, closingBracket - 5);

        // open image and check if it exists
        std::ifstream image(imageName);

        if (!image.is_open()) {
            // TODO: somehow return error and abort "compilation"
            return "failed to open image file";
        }

        std::string content(std::istreambuf_iterator<char>(image), {});

        // convert it into a base64
        const auto &encoded = base64_encode(content, false);

        // return image tag with base64 data
        const auto &fileExt = getFileExt(imageName);
        const auto &mimeType = getMimeType(fileExt);

        return std::format("<img src=\"data:{};base64,{}\"/>", mimeType, encoded);
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

    // try to create output file
    std::ofstream output(outputName);
    if (!output.is_open()) {
        std::print("Failed to open output file! Do I have permissions?\n");
        return 1;
    }

    // add html skeleton
    output << "<!DOCTYPE html>\n<html>\n<body>\n";

    // read file line by line
    std::string line;
    while (std::getline(source, line)) {
        const auto &html = parseLine(line);

        if (!html.empty()) {
            output << html << "\n";
        }
    }

    // finish html doc
    output << "</body>\n</html>\n";

    std::print("Finished transpiling!\n");

    return 0;
}
