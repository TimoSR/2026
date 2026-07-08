#ifdef _WIN32
#define NOMINMAX
#include <windows.h>
#endif

#include <sql.h>
#include <sqlext.h>

#include <string>
#include <vector>

#ifdef _WIN32
#pragma comment(lib, "odbc32.lib")
#endif

using namespace std;

string odbcDiagnostics(SQLSMALLINT handleType, SQLHANDLE handle) {
    string diagnostics = "";

    SQLSMALLINT recordNumber = 1;
    while (true) {
        SQLCHAR stateBuffer[7] = {};
        SQLCHAR messageBuffer[512] = {};
        SQLINTEGER nativeCode = 0;
        SQLSMALLINT messageLength = 0;

        SQLRETURN recordResult = SQLGetDiagRecA(
            handleType,
            handle,
            recordNumber,
            stateBuffer,
            &nativeCode,
            messageBuffer,
            sizeof(messageBuffer),
            &messageLength
        );

        if (recordResult == SQL_NO_DATA) {
            break;
        }

        if (!SQL_SUCCEEDED(recordResult)) {
            break;
        }

        if (!diagnostics.empty()) {
            diagnostics += " | ";
        }

        diagnostics += "[";
        diagnostics += reinterpret_cast<char*>(stateBuffer);
        diagnostics += "] ";
        diagnostics += string(reinterpret_cast<char*>(messageBuffer), static_cast<size_t>(messageLength));

        recordNumber += 1;
    }

    if (diagnostics.empty()) {
        return "No ODBC diagnostics available.";
    }

    return diagnostics;
}

class ScalarQueryResult {
private:
    bool success = false;
    int value = -1;
    string errorMessage = "";

public:
    ScalarQueryResult(bool successValue, int valueValue, string errorMessageValue)
        : success(successValue), value(valueValue), errorMessage(errorMessageValue) {
    }

    bool isSuccess() const {
        return success;
    }

    int getValue() const {
        return value;
    }

    string getErrorMessage() const {
        return errorMessage;
    }
};

class OdbcSession {
private:
    SQLHENV environmentHandle = SQL_NULL_HENV;
    SQLHDBC connectionHandle = SQL_NULL_HDBC;

public:
    ~OdbcSession() {
        close();
    }

    bool open(string connectionString, string& errorMessage) {
        close();

        SQLRETURN environmentResult = SQLAllocHandle(SQL_HANDLE_ENV, SQL_NULL_HANDLE, &environmentHandle);
        if (!SQL_SUCCEEDED(environmentResult)) {
            errorMessage = "Could not allocate ODBC environment handle.";
            close();
            return false;
        }

        SQLRETURN odbcVersionResult = SQLSetEnvAttr(
            environmentHandle,
            SQL_ATTR_ODBC_VERSION,
            reinterpret_cast<SQLPOINTER>(SQL_OV_ODBC3),
            0
        );
        if (!SQL_SUCCEEDED(odbcVersionResult)) {
            errorMessage = odbcDiagnostics(SQL_HANDLE_ENV, environmentHandle);
            close();
            return false;
        }

        SQLRETURN connectionResult = SQLAllocHandle(SQL_HANDLE_DBC, environmentHandle, &connectionHandle);
        if (!SQL_SUCCEEDED(connectionResult)) {
            errorMessage = odbcDiagnostics(SQL_HANDLE_ENV, environmentHandle);
            close();
            return false;
        }

        vector<SQLCHAR> connectionBuffer(connectionString.begin(), connectionString.end());
        connectionBuffer.push_back('\0');

        SQLCHAR outConnectionBuffer[1024] = {};
        SQLSMALLINT outConnectionLength = 0;

        SQLRETURN connectResult = SQLDriverConnectA(
            connectionHandle,
            nullptr,
            connectionBuffer.data(),
            SQL_NTS,
            outConnectionBuffer,
            sizeof(outConnectionBuffer),
            &outConnectionLength,
            SQL_DRIVER_NOPROMPT
        );

        if (!SQL_SUCCEEDED(connectResult)) {
            errorMessage = odbcDiagnostics(SQL_HANDLE_DBC, connectionHandle);
            close();
            return false;
        }

        return true;
    }

    ScalarQueryResult querySingleIntValue(string sqlQuery) {
        if (connectionHandle == SQL_NULL_HDBC) {
            return ScalarQueryResult(false, -1, "Session is not open.");
        }

        SQLHSTMT statementHandle = SQL_NULL_HSTMT;
        SQLRETURN statementResult = SQLAllocHandle(SQL_HANDLE_STMT, connectionHandle, &statementHandle);
        if (!SQL_SUCCEEDED(statementResult)) {
            return ScalarQueryResult(false, -1, odbcDiagnostics(SQL_HANDLE_DBC, connectionHandle));
        }

        vector<SQLCHAR> sqlBuffer(sqlQuery.begin(), sqlQuery.end());
        sqlBuffer.push_back('\0');

        SQLRETURN executeResult = SQLExecDirectA(statementHandle, sqlBuffer.data(), SQL_NTS);
        if (!SQL_SUCCEEDED(executeResult)) {
            string error = odbcDiagnostics(SQL_HANDLE_STMT, statementHandle);
            SQLFreeHandle(SQL_HANDLE_STMT, statementHandle);
            return ScalarQueryResult(false, -1, error);
        }

        SQLRETURN fetchResult = SQLFetch(statementHandle);
        if (fetchResult == SQL_NO_DATA) {
            SQLFreeHandle(SQL_HANDLE_STMT, statementHandle);
            return ScalarQueryResult(false, -1, "Query returned no rows.");
        }
        if (!SQL_SUCCEEDED(fetchResult)) {
            string error = odbcDiagnostics(SQL_HANDLE_STMT, statementHandle);
            SQLFreeHandle(SQL_HANDLE_STMT, statementHandle);
            return ScalarQueryResult(false, -1, error);
        }

        SQLCHAR valueBuffer[64] = {};
        SQLLEN valueIndicator = 0;
        SQLRETURN dataResult = SQLGetData(
            statementHandle,
            1,
            SQL_C_CHAR,
            valueBuffer,
            sizeof(valueBuffer),
            &valueIndicator
        );
        if (!SQL_SUCCEEDED(dataResult)) {
            string error = odbcDiagnostics(SQL_HANDLE_STMT, statementHandle);
            SQLFreeHandle(SQL_HANDLE_STMT, statementHandle);
            return ScalarQueryResult(false, -1, error);
        }

        SQLFreeHandle(SQL_HANDLE_STMT, statementHandle);

        try {
            int parsedValue = stoi(string(reinterpret_cast<char*>(valueBuffer)));
            return ScalarQueryResult(true, parsedValue, "");
        } catch (...) {
            return ScalarQueryResult(false, -1, "Could not parse integer from query result.");
        }
    }

    void close() {
        if (connectionHandle != SQL_NULL_HDBC) {
            SQLDisconnect(connectionHandle);
            SQLFreeHandle(SQL_HANDLE_DBC, connectionHandle);
            connectionHandle = SQL_NULL_HDBC;
        }

        if (environmentHandle != SQL_NULL_HENV) {
            SQLFreeHandle(SQL_HANDLE_ENV, environmentHandle);
            environmentHandle = SQL_NULL_HENV;
        }
    }
};

vector<string> postgresOdbcDriverCandidates() {
    vector<string> candidates;
    candidates.push_back("PostgreSQL Unicode(x64)");
    candidates.push_back("PostgreSQL Unicode");
    candidates.push_back("PostgreSQL ANSI(x64)");
    candidates.push_back("PostgreSQL ANSI");
    return candidates;
}

vector<string> mysqlOdbcDriverCandidates() {
    vector<string> candidates;
    candidates.push_back("MySQL ODBC 8.4 Unicode Driver");
    candidates.push_back("MySQL ODBC 8.3 Unicode Driver");
    candidates.push_back("MySQL ODBC 8.0 Unicode Driver");
    return candidates;
}

string postgresDsnLessConnectionString(
    string driverName,
    string host,
    int port,
    string database,
    string user,
    string password
) {
    return "Driver={" + driverName + "};Server=" + host + ";Port=" + to_string(port) +
           ";Database=" + database + ";Uid=" + user + ";Pwd=" + password + ";";
}

string mysqlDsnLessConnectionString(
    string driverName,
    string host,
    int port,
    string database,
    string user,
    string password
) {
    return "Driver={" + driverName + "};Server=" + host + ";Port=" + to_string(port) +
           ";Database=" + database + ";User=" + user + ";Password=" + password + ";";
}

class DatabaseProbeOutcome {
private:
    bool didConnectAndQuery = false;
    int seedEmailCount = -1;
    string usedDriver = "";
    string errorMessage = "";

public:
    DatabaseProbeOutcome() = default;

    DatabaseProbeOutcome(bool successValue, int countValue, string driverValue, string errorValue)
        : didConnectAndQuery(successValue), seedEmailCount(countValue), usedDriver(driverValue), errorMessage(errorValue) {
    }

    bool isSuccess() const {
        return didConnectAndQuery;
    }

    int getSeedEmailCount() const {
        return seedEmailCount;
    }

    string getUsedDriver() const {
        return usedDriver;
    }

    string getErrorMessage() const {
        return errorMessage;
    }
};

DatabaseProbeOutcome probeSeedEmailCountViaOdbc(
    vector<string> driverCandidates,
    bool isPostgres,
    string host,
    int port,
    string database,
    string user,
    string password
) {
    string query = "SELECT COUNT(*) FROM users WHERE email='seed@example.com';";
    string lastError = "No ODBC driver candidate was accepted.";

    for (string driverName : driverCandidates) {
        string connectionString = "";
        if (isPostgres) {
            connectionString = postgresDsnLessConnectionString(driverName, host, port, database, user, password);
        } else {
            connectionString = mysqlDsnLessConnectionString(driverName, host, port, database, user, password);
        }

        OdbcSession session;
        string openError = "";
        if (!session.open(connectionString, openError)) {
            lastError = "Driver '" + driverName + "' open failed: " + openError;
            continue;
        }

        ScalarQueryResult queryResult = session.querySingleIntValue(query);
        session.close();

        if (queryResult.isSuccess()) {
            return DatabaseProbeOutcome(true, queryResult.getValue(), driverName, "");
        }

        lastError = "Driver '" + driverName + "' query failed: " + queryResult.getErrorMessage();
    }

    return DatabaseProbeOutcome(false, -1, "", lastError);
}

DatabaseProbeOutcome probePostgresSeedUserViaOdbc() {
    return probeSeedEmailCountViaOdbc(
        postgresOdbcDriverCandidates(),
        true,
        "127.0.0.1",
        55432,
        "app",
        "app",
        "app"
    );
}

DatabaseProbeOutcome probeMySqlSeedUserViaOdbc() {
    return probeSeedEmailCountViaOdbc(
        mysqlOdbcDriverCandidates(),
        false,
        "127.0.0.1",
        53306,
        "app",
        "app",
        "app"
    );
}

bool isProbeOutcomeWellFormed(DatabaseProbeOutcome outcome) {
    if (outcome.isSuccess()) {
        return outcome.getSeedEmailCount() >= 0 && !outcome.getUsedDriver().empty();
    }

    return outcome.getSeedEmailCount() == -1 && !outcome.getErrorMessage().empty();
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    DatabaseProbeOutcome postgresProbe = probePostgresSeedUserViaOdbc();
    DatabaseProbeOutcome mysqlProbe = probeMySqlSeedUserViaOdbc();

    cout << "[postgres odbc]\n";
    cout << "success: " << postgresProbe.isSuccess() << "\n";
    cout << "driver: " << postgresProbe.getUsedDriver() << "\n";
    cout << "seed count: " << postgresProbe.getSeedEmailCount() << "\n";
    if (!postgresProbe.isSuccess()) {
        cout << "error: " << postgresProbe.getErrorMessage() << "\n";
    }

    cout << "\n[mysql odbc]\n";
    cout << "success: " << mysqlProbe.isSuccess() << "\n";
    cout << "driver: " << mysqlProbe.getUsedDriver() << "\n";
    cout << "seed count: " << mysqlProbe.getSeedEmailCount() << "\n";
    if (!mysqlProbe.isSuccess()) {
        cout << "error: " << mysqlProbe.getErrorMessage() << "\n";
    }

    return 0;
}
#endif
