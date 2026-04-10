#include <iostream>
#include <sstream>
#include <string>

using namespace std;

string formatWithCoutStyle(string name, int score) {
    ostringstream output;
    output << "User: " << name << ", Score: " << score;
    return output.str();
}

void writeLineWithCout(ostream& output, string label, int value) {
    output << label << ": " << value << "\n";
}

#ifdef RUN_DEMO
int main() {
    cout << formatWithCoutStyle("Nora", 95) << "\n";
    writeLineWithCout(cout, "Lives", 3);
    return 0;
}
#endif
