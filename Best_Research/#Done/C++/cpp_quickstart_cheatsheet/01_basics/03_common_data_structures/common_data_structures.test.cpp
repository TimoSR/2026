#include <cassert>
#include <map>
#include <queue>
#include <set>
#include <string>
#include <vector>

using namespace std;

#include "common_data_structures.cpp"

int main() {
    vector<string> tasks = buildTaskList();
    assert(tasks.size() == 4);
    assert(tasks[0] == "Read");
    assert(tasks[3] == "Ship");

    map<string, int> counts = buildWordCount();
    assert(counts["cpp"] == 2);
    assert(counts["rust"] == 1);

    set<string> tags = buildUniqueTags();
    assert(tags.count("backend") == 1);
    assert(tags.count("cpp") == 1);
    assert(tags.size() == 3);

    queue<string> tickets = buildSupportQueue();
    assert(tickets.front() == "ticket-1");
    tickets.pop();
    assert(tickets.front() == "ticket-2");

    return 0;
}
