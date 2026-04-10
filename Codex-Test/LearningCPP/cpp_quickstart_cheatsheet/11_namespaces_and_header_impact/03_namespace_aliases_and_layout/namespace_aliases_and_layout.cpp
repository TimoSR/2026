#include <string>
#include <vector>

using namespace std;

namespace company {
    namespace platform {
        namespace notifications {
            class EmailTemplate {
            public:
                string buildSubject(string userName) {
                    return "Welcome " + userName;
                }
            };
        }
    }
}

namespace notifications = company::platform::notifications;

string buildWelcomeSubject(string userName) {
    notifications::EmailTemplate templateBuilder;
    return templateBuilder.buildSubject(userName);
}

string buildWelcomeSubjectWithoutAlias(string userName) {
    company::platform::notifications::EmailTemplate templateBuilder;
    return templateBuilder.buildSubject(userName);
}

vector<string> buildWelcomeSubjectsBatch(vector<string> userNames) {
    vector<string> subjects;
    for (string userName : userNames) {
        subjects.push_back(buildWelcomeSubject(userName));
    }
    return subjects;
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    cout << "[namespace_aliases_and_layout]\n";
    cout << "with alias: " << buildWelcomeSubject("Nora") << "\n";
    cout << "without alias: " << buildWelcomeSubjectWithoutAlias("Nora") << "\n";

    vector<string> subjects = buildWelcomeSubjectsBatch({"Nora", "Ava"});
    for (string subject : subjects) {
        cout << subject << "\n";
    }
    return 0;
}
#endif
