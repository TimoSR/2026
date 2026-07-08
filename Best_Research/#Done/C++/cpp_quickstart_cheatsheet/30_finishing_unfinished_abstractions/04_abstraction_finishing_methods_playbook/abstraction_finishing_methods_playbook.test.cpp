#include <cassert>
#include <vector>

using namespace std;

#include "abstraction_finishing_methods_playbook.cpp"

bool containsName(vector<string> names, string expected) {
    for (string name : names) {
        if (name == expected) {
            return true;
        }
    }
    return false;
}

int main() {
    {
        vector<AbstractionMethod> methods = finishingMethods();
        assert(methods.size() == 5);
        assert(methods[0].getName() == "Facade wrapper");
    }

    {
        vector<string> asyncPicks = recommendedMethodsFor("async");
        assert(asyncPicks.size() == 3);
        assert(containsName(asyncPicks, "Facade wrapper"));
        assert(containsName(asyncPicks, "Profile defaults"));
        assert(containsName(asyncPicks, "Error normalization"));
    }

    {
        vector<string> socketPicks = recommendedMethodsFor("sockets");
        assert(containsName(socketPicks, "Adapter boundary"));
        assert(containsName(socketPicks, "Error normalization"));
        assert(containsName(socketPicks, "Lifecycle manager"));
    }

    return 0;
}
