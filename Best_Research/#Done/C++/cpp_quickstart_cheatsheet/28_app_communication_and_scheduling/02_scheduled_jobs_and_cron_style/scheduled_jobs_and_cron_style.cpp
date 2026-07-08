#include <string>
#include <vector>

using namespace std;

class IntervalJob {
private:
    string name = "";
    int everyMinutes = 1;

public:
    IntervalJob(string nameValue, int everyMinutesValue) : name(nameValue), everyMinutes(everyMinutesValue) {
    }

    string getName() const {
        return name;
    }

    int getEveryMinutes() const {
        return everyMinutes;
    }
};

bool shouldRunAtMinute(IntervalJob job, int minuteOfDay) {
    if (job.getEveryMinutes() <= 0) {
        return false;
    }
    return (minuteOfDay % job.getEveryMinutes()) == 0;
}

vector<string> jobsDueNow(vector<IntervalJob> jobs, int minuteOfDay) {
    vector<string> dueJobs;
    for (IntervalJob job : jobs) {
        if (shouldRunAtMinute(job, minuteOfDay)) {
            dueJobs.push_back(job.getName());
        }
    }
    return dueJobs;
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    vector<IntervalJob> jobs;
    jobs.push_back(IntervalJob("cleanup_temp_files", 15));
    jobs.push_back(IntervalJob("sync_orders", 5));
    jobs.push_back(IntervalJob("send_daily_report", 1440));

    int minuteOfDay = 30;
    vector<string> due = jobsDueNow(jobs, minuteOfDay);

    cout << "[jobsDueNow at minute 30]\n";
    for (string jobName : due) {
        cout << jobName << "\n";
    }
    return 0;
}
#endif
