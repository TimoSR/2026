#include <cassert>
#include <vector>

using namespace std;

#include "polling_patterns.cpp"

int main() {
    {
        JobStatusStore store;
        store.setStatus("job-1", "completed");
        PollingResult result = pollUntilDone(store, "job-1", 5);
        assert(result.isCompleted() == true);
        assert(result.getChecks() == 1);
    }

    {
        JobStatusStore store;
        store.setStatus("job-2", "running");
        PollingResult result = pollUntilDone(store, "job-2", 3);
        assert(result.isCompleted() == false);
        assert(result.getChecks() == 3);
        vector<string> statuses = result.getObservedStatuses();
        assert(statuses.size() == 3);
        assert(statuses[0] == "running");
    }

    return 0;
}
