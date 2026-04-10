#include <cassert>
#include <string>
#include <vector>

#include "postgres_mysql_odbc_calls.cpp"

int main() {
    std::vector<std::string> postgresDrivers = postgresOdbcDriverCandidates();
    std::vector<std::string> mysqlDrivers = mysqlOdbcDriverCandidates();
    assert(postgresDrivers.empty() == false);
    assert(mysqlDrivers.empty() == false);

    std::string postgresConnection = postgresDsnLessConnectionString(
        postgresDrivers[0], "127.0.0.1", 55432, "app", "app", "app"
    );
    assert(postgresConnection.find("Driver={") == 0);
    assert(postgresConnection.find("Server=127.0.0.1;") != std::string::npos);
    assert(postgresConnection.find("Port=55432;") != std::string::npos);

    std::string mysqlConnection = mysqlDsnLessConnectionString(
        mysqlDrivers[0], "127.0.0.1", 53306, "app", "app", "app"
    );
    assert(mysqlConnection.find("Driver={") == 0);
    assert(mysqlConnection.find("Server=127.0.0.1;") != std::string::npos);
    assert(mysqlConnection.find("Port=53306;") != std::string::npos);

    DatabaseProbeOutcome postgresProbe = probePostgresSeedUserViaOdbc();
    DatabaseProbeOutcome mysqlProbe = probeMySqlSeedUserViaOdbc();

    assert(isProbeOutcomeWellFormed(postgresProbe) == true);
    assert(isProbeOutcomeWellFormed(mysqlProbe) == true);

    if (postgresProbe.isSuccess()) {
        assert(postgresProbe.getSeedEmailCount() >= 0);
    }
    if (mysqlProbe.isSuccess()) {
        assert(mysqlProbe.getSeedEmailCount() >= 0);
    }

    return 0;
}
