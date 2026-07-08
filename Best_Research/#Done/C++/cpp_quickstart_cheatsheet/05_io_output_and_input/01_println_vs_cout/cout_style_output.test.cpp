#include <cassert>
#include <sstream>
#include <string>

using namespace std;

#include "cout_style_output.cpp"

int main() {
    string formattedText = formatWithCoutStyle("Nora", 95);
    assert(formattedText == "User: Nora, Score: 95");

    ostringstream output;
    writeLineWithCout(output, "Lives", 3);
    assert(output.str() == "Lives: 3\n");

    return 0;
}
