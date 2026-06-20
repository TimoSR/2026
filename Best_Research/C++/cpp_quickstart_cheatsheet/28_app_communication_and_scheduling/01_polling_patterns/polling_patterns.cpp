#include <map>
#include <string>
#include <vector>

using namespace std;

class JobStatusStore {
private:
    map<string, string> statuses;

public:
    void setStatus(string jobId, string status) {
        statuses[jobId] = status;
    }

    string getStatus(string jobId) const {
        map<string, string>::const_iterator found = statuses.find(jobId);
        if (found == statuses.end()) {
            return "missing";
        }
        return found->second;
    }
};

class PollingResult {
private:
    bool completed = false;
    int checks = 0;
    vector<string> observedStatuses;

public:
    PollingResult(bool completedValue, int checksValue, vector<string> statusesValue)
        : completed(completedValue), checks(checksValue), observedStatuses(statusesValue) {
    }

    bool isCompleted() const {
        return completed;
    }

    int getChecks() const {
        return checks;
    }

    vector<string> getObservedStatuses() const {
        return observedStatuses;
    }
};

PollingResult pollUntilDone(JobStatusStore& store, string jobId, int maxChecks) {
    vector<string> observedStatuses;

    for (int checkNumber = 1; checkNumber <= maxChecks; checkNumber += 1) {
        string status = store.getStatus(jobId);
        observedStatuses.push_back(status);
        if (status == "completed") {
            return PollingResult(true, checkNumber, observedStatuses);
        }
    }

    return PollingResult(false, maxChecks, observedStatuses);
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    JobStatusStore store;
    store.setStatus("export-1", "running");

    PollingResult before = pollUntilDone(store, "export-1", 3);
    cout << "[pollUntilDone / before completion]\n";
    cout << "completed: " << before.isCompleted() << ", checks: " << before.getChecks() << "\n\n";

    store.setStatus("export-1", "completed");
    PollingResult after = pollUntilDone(store, "export-1", 3);
    cout << "[pollUntilDone / after completion]\n";
    cout << "completed: " << after.isCompleted() << ", checks: " << after.getChecks() << "\n";
    return 0;
}
#endif
