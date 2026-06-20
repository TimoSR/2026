#include <map>
#include <queue>
#include <set>
#include <string>
#include <vector>

using namespace std;

vector<string> buildTaskList() {
    vector<string> tasks = {"Read", "Code", "Test"};
    tasks.push_back("Ship");
    return tasks;
}

map<string, int> buildWordCount() {
    map<string, int> counts;
    counts["cpp"] = 2;
    counts["rust"] = 1;
    return counts;
}

set<string> buildUniqueTags() {
    set<string> tags = {"backend", "cpp", "backend", "testing"};
    return tags;
}

queue<string> buildSupportQueue() {
    queue<string> tickets;
    tickets.push("ticket-1");
    tickets.push("ticket-2");
    tickets.push("ticket-3");
    return tickets;
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    using namespace std;

    vector<string> tasks = buildTaskList();
    cout << "Task count: " << tasks.size() << "\n";

    map<string, int> counts = buildWordCount();
    cout << "cpp count: " << counts["cpp"] << "\n";

    set<string> tags = buildUniqueTags();
    cout << "Unique tags: " << tags.size() << "\n";

    queue<string> tickets = buildSupportQueue();
    cout << "First ticket: " << tickets.front() << "\n";
    return 0;
}
#endif
