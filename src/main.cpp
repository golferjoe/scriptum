#include <cstdio>
#include <fstream>
#include <iostream>
#include <iterator>

int main(int argc, char *argv[]) {
    // check if enough arguments were provided
    if (argc < 2) {
        printf("No file to transpile provided!\n");
        return 1;
    }

    // extract source file and check if it exists
    const auto fileName = argv[1];

    std::ifstream file(fileName);
    if (!file.is_open()) {
        printf("Failed to open source file! Does the file exists?\n");
        return 1;
    }

    // read file contents
    std::string content((std::istreambuf_iterator<char>(file)),
                        (std::istreambuf_iterator<char>()));

    std::cout << content << std::endl;

    return 0;
}
