#include <string>
#include <vector>

using namespace std;

string postgresConnectionString(string host, int port, string database, string user, string password) {
    return "host=" + host + " port=" + to_string(port) + " dbname=" + database +
           " user=" + user + " password=" + password;
}

string mysqlConnectionString(string host, int port, string database, string user, string password) {
    return "mysql://" + user + ":" + password + "@" + host + ":" + to_string(port) + "/" + database;
}

string findUserByEmailQuery() {
    return "SELECT id, email FROM users WHERE email = ?";
}

class IUserRepository {
public:
    virtual ~IUserRepository() = default;
    virtual bool emailExists(string email) = 0;
};

class FakeUserRepository : public IUserRepository {
private:
    vector<string> existingEmails;

public:
    FakeUserRepository(vector<string> existingEmailsParam)
        : existingEmails(existingEmailsParam) {
    }

    bool emailExists(string email) override {
        for (string existingEmail : existingEmails) {
            if (existingEmail == email) {
                return true;
            }
        }
        return false;
    }
};

bool canRegisterEmail(IUserRepository* userRepository, string email) {
    if (email.empty()) {
        return false;
    }
    return !userRepository->emailExists(email);
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    using namespace std;

    cout << postgresConnectionString("localhost", 5432, "app", "nora", "secret") << "\n";
    cout << mysqlConnectionString("localhost", 3306, "app", "nora", "secret") << "\n";
    cout << findUserByEmailQuery() << "\n";

    FakeUserRepository repo({"nora@example.com"});
    cout << canRegisterEmail(&repo, "ava@example.com") << "\n";
    return 0;
}
#endif
