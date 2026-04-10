#include <cassert>
#include <vector>

using namespace std;

#include "namespace_aliases_and_layout.cpp"

int main() {
    assert(buildWelcomeSubject("Nora") == "Welcome Nora");
    assert(buildWelcomeSubjectWithoutAlias("Nora") == "Welcome Nora");

    vector<string> subjects = buildWelcomeSubjectsBatch({"Nora", "Ava"});
    assert(subjects.size() == 2);
    assert(subjects[0] == "Welcome Nora");
    assert(subjects[1] == "Welcome Ava");
    return 0;
}
