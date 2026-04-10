#include <string>

using namespace std;

class ReportRenderOptions {
private:
    bool includeCharts = true;
    bool includeSummary = true;
    string theme = "light";

public:
    ReportRenderOptions() = default;

    ReportRenderOptions(bool charts, bool summary, string themeValue)
        : includeCharts(charts), includeSummary(summary), theme(themeValue) {
    }

    bool getIncludeCharts() const {
        return includeCharts;
    }

    bool getIncludeSummary() const {
        return includeSummary;
    }

    string getTheme() const {
        return theme;
    }
};

string renderReport(string reportName, ReportRenderOptions options) {
    return "report=" + reportName +
           ", charts=" + (options.getIncludeCharts() ? "yes" : "no") +
           ", summary=" + (options.getIncludeSummary() ? "yes" : "no") +
           ", theme=" + options.getTheme();
}

// Overload for easy standard call.
string renderReport(string reportName) {
    return renderReport(reportName, ReportRenderOptions());
}

ReportRenderOptions chartsOffDarkTheme() {
    return ReportRenderOptions(false, true, "dark");
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    cout << "[default overload]\n";
    cout << renderReport("weekly_sales") << "\n\n";

    cout << "[profile/options overload]\n";
    cout << renderReport("weekly_sales", chartsOffDarkTheme()) << "\n";
    return 0;
}
#endif
