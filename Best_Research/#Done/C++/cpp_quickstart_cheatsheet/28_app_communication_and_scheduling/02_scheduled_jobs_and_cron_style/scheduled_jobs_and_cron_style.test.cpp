#include <cassert>
#include <vector>

using namespace std;

#include "scheduled_jobs_and_cron_style.cpp"

int main() {
    {
        IntervalJob fiveMinuteJob("sync", 5);
        assert(shouldRunAtMinute(fiveMinuteJob, 10) == true);
        assert(shouldRunAtMinute(fiveMinuteJob, 11) == false);
    }

    {
        vector<IntervalJob> jobs;
        jobs.push_back(IntervalJob("job15", 15));
        jobs.push_back(IntervalJob("job10", 10));
        jobs.push_back(IntervalJob("job7", 7));

        vector<string> due = jobsDueNow(jobs, 30);
        assert(due.size() == 2);
        assert(due[0] == "job15");
        assert(due[1] == "job10");
    }

    return 0;
}
