#include <string>

using namespace std;

class ExportOptions {
private:
    bool includeHeaders = true;
    bool compressOutput = true;
    int parallelWorkers = 2;
    string format = "csv";

public:
    ExportOptions() = default;

    ExportOptions(bool headers, bool compress, int workers, string formatValue)
        : includeHeaders(headers), compressOutput(compress), parallelWorkers(workers), format(formatValue) {
    }

    bool getIncludeHeaders() const {
        return includeHeaders;
    }

    bool getCompressOutput() const {
        return compressOutput;
    }

    int getParallelWorkers() const {
        return parallelWorkers;
    }

    string getFormat() const {
        return format;
    }
};

ExportOptions buildSafeProfile() {
    return ExportOptions(true, true, 2, "csv");
}

ExportOptions buildFastProfile() {
    return ExportOptions(false, false, 8, "binary");
}

ExportOptions buildDebugProfile() {
    return ExportOptions(true, false, 1, "json");
}

string describeExportRun(string datasetName, ExportOptions options) {
    return "dataset=" + datasetName +
           ", format=" + options.getFormat() +
           ", workers=" + to_string(options.getParallelWorkers()) +
           ", headers=" + (options.getIncludeHeaders() ? "yes" : "no") +
           ", compress=" + (options.getCompressOutput() ? "yes" : "no");
}

string inferProfileName(ExportOptions options) {
    if (options.getIncludeHeaders() && options.getCompressOutput() && options.getParallelWorkers() == 2 && options.getFormat() == "csv") {
        return "safe";
    }

    if (!options.getIncludeHeaders() && !options.getCompressOutput() && options.getParallelWorkers() >= 8 && options.getFormat() == "binary") {
        return "fast";
    }

    if (options.getIncludeHeaders() && !options.getCompressOutput() && options.getParallelWorkers() == 1 && options.getFormat() == "json") {
        return "debug";
    }

    return "custom";
}

bool isInteractiveFriendly(ExportOptions options) {
    bool avoidsBinaryOnly = options.getFormat() != "binary";
    bool includesReadableHeaders = options.getIncludeHeaders();
    bool boundedWorkers = options.getParallelWorkers() <= 4;
    return avoidsBinaryOnly && includesReadableHeaders && boundedWorkers;
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    cout << "[safe profile]\n";
    cout << describeExportRun("orders_2026", buildSafeProfile()) << "\n\n";
    cout << "inferred name: " << inferProfileName(buildSafeProfile()) << "\n";
    cout << "interactive friendly: " << isInteractiveFriendly(buildSafeProfile()) << "\n\n";

    cout << "[fast profile]\n";
    cout << describeExportRun("orders_2026", buildFastProfile()) << "\n\n";
    cout << "inferred name: " << inferProfileName(buildFastProfile()) << "\n";
    cout << "interactive friendly: " << isInteractiveFriendly(buildFastProfile()) << "\n\n";

    cout << "[debug profile]\n";
    cout << describeExportRun("orders_2026", buildDebugProfile()) << "\n";
    cout << "inferred name: " << inferProfileName(buildDebugProfile()) << "\n";
    cout << "interactive friendly: " << isInteractiveFriendly(buildDebugProfile()) << "\n";
    return 0;
}
#endif
