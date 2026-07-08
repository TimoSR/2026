#include <cassert>

#include "file_read_write_with_createfilew.cpp"

int main() {
    std::string filePath = tempDirectoryPathWin32() + "win32_file_io_test.txt";
    bool writeOk = writeTextFileWithCreateFileW(filePath, "abc123");

    assert(writeOk == true);
    assert(readTextFileWithCreateFileW(filePath) == "abc123");
    return 0;
}
