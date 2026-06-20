#include <cassert>
using namespace std;

#include "preconfigured_parameter_profiles.cpp"

int main() {
    {
        ExportOptions safe = buildSafeProfile();
        assert(safe.getFormat() == "csv");
        assert(safe.getCompressOutput() == true);
        assert(safe.getParallelWorkers() == 2);
    }

    {
        ExportOptions fast = buildFastProfile();
        assert(fast.getFormat() == "binary");
        assert(fast.getCompressOutput() == false);
        assert(fast.getParallelWorkers() == 8);
    }

    {
        string description = describeExportRun("orders", buildDebugProfile());
        assert(description.find("format=json") != string::npos);
        assert(description.find("workers=1") != string::npos);
    }

    {
        assert(inferProfileName(buildSafeProfile()) == "safe");
        assert(inferProfileName(buildFastProfile()) == "fast");
        assert(inferProfileName(buildDebugProfile()) == "debug");
        assert(inferProfileName(ExportOptions(true, false, 3, "csv")) == "custom");
    }

    {
        assert(isInteractiveFriendly(buildSafeProfile()) == true);
        assert(isInteractiveFriendly(buildFastProfile()) == false);
        assert(isInteractiveFriendly(buildDebugProfile()) == true);
    }

    return 0;
}
