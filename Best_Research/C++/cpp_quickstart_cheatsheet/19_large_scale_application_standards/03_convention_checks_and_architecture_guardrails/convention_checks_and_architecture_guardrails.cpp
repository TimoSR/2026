#include <string>
#include <utility>
#include <vector>

using namespace std;

enum class ETypeCategory {
    Actor,
    UObjectLike,
    Interface,
    StructLike,
    EnumLike,
    TemplateLike,
    Unknown
};

struct FConventionViolation {
    string symbolName = "";
    string reason = "";
};

ETypeCategory detectTypeCategory(string symbolName) {
    if (symbolName.empty()) {
        return ETypeCategory::Unknown;
    }

    char prefix = symbolName[0];
    if (prefix == 'A') {
        return ETypeCategory::Actor;
    }
    if (prefix == 'U') {
        return ETypeCategory::UObjectLike;
    }
    if (prefix == 'I') {
        return ETypeCategory::Interface;
    }
    if (prefix == 'F') {
        return ETypeCategory::StructLike;
    }
    if (prefix == 'E') {
        return ETypeCategory::EnumLike;
    }
    if (prefix == 'T') {
        return ETypeCategory::TemplateLike;
    }

    return ETypeCategory::Unknown;
}

vector<FConventionViolation> findNamingViolations(vector<string> symbolNames) {
    vector<FConventionViolation> violations;

    for (string symbolName : symbolNames) {
        ETypeCategory typeCategory = detectTypeCategory(symbolName);
        if (typeCategory == ETypeCategory::Unknown) {
            FConventionViolation violation;
            violation.symbolName = symbolName;
            violation.reason = "Missing expected prefix (A/U/I/F/E/T).";
            violations.push_back(violation);
        }
    }

    return violations;
}

bool isAllowedDependency(string fromLayer, string toLayer) {
    if (fromLayer == "ui") {
        return toLayer == "application";
    }
    if (fromLayer == "application") {
        return toLayer == "domain";
    }
    if (fromLayer == "infrastructure") {
        return toLayer == "application" || toLayer == "domain";
    }
    if (fromLayer == "domain") {
        return false;
    }
    return false;
}

int architectureViolationCount(vector<pair<string, string>> dependencies) {
    int violationCount = 0;
    for (pair<string, string> dependency : dependencies) {
        if (!isAllowedDependency(dependency.first, dependency.second)) {
            violationCount += 1;
        }
    }
    return violationCount;
}

bool hasArchitectureViolations(vector<pair<string, string>> dependencies) {
    return architectureViolationCount(dependencies) > 0;
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    vector<string> symbolNames;
    symbolNames.push_back("APlayerCharacter");
    symbolNames.push_back("UInventoryComponent");
    symbolNames.push_back("IInventoryGateway");
    symbolNames.push_back("PlayerController");

    vector<FConventionViolation> violations = findNamingViolations(symbolNames);

    cout << "[naming guardrails]\n";
    cout << "violations: " << violations.size() << "\n";
    for (FConventionViolation violation : violations) {
        cout << "- " << violation.symbolName << " -> " << violation.reason << "\n";
    }

    cout << "\n[dependency guardrails]\n";
    cout << "ui -> application: " << isAllowedDependency("ui", "application") << "\n";
    cout << "application -> infrastructure: " << isAllowedDependency("application", "infrastructure") << "\n";
    vector<pair<string, string>> dependencies;
    dependencies.push_back(make_pair("ui", "application"));
    dependencies.push_back(make_pair("application", "domain"));
    dependencies.push_back(make_pair("application", "infrastructure"));
    cout << "violation count: " << architectureViolationCount(dependencies) << "\n";
    return 0;
}
#endif
