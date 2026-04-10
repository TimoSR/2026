#include <cassert>
#include <string>

using namespace std;

#include "postgres_mysql_patterns.cpp"

int main() {
    string pg = postgresConnectionString("localhost", 5432, "app", "nora", "secret");
    assert(pg == "host=localhost port=5432 dbname=app user=nora password=secret");

    string my = mysqlConnectionString("localhost", 3306, "app", "nora", "secret");
    assert(my == "mysql://nora:secret@localhost:3306/app");

    string query = findUserByEmailQuery();
    assert(query == "SELECT id, email FROM users WHERE email = ?");

    FakeUserRepository repo({"nora@example.com"});
    assert(canRegisterEmail(&repo, "ava@example.com") == true);
    assert(canRegisterEmail(&repo, "nora@example.com") == false);
    assert(canRegisterEmail(&repo, "") == false);

    return 0;
}
