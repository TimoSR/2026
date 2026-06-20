#include <cassert>
#include <utility>
#include <vector>

using namespace std;

#include "convention_checks_and_architecture_guardrails.cpp"

int main() {
    vector<string> symbolNames;
    symbolNames.push_back("APlayerCharacter");
    symbolNames.push_back("FGameState");
    symbolNames.push_back("PlayerManager");

    vector<FConventionViolation> violations = findNamingViolations(symbolNames);
    assert(violations.size() == 1);
    assert(violations[0].symbolName == "PlayerManager");

    assert(detectTypeCategory("IEmailGateway") == ETypeCategory::Interface);
    assert(detectTypeCategory("EGameMode") == ETypeCategory::EnumLike);

    assert(isAllowedDependency("ui", "application") == true);
    assert(isAllowedDependency("application", "domain") == true);
    assert(isAllowedDependency("application", "infrastructure") == false);

    vector<pair<string, string>> dependencies;
    dependencies.push_back(make_pair("ui", "application"));
    dependencies.push_back(make_pair("application", "domain"));
    dependencies.push_back(make_pair("application", "infrastructure"));

    assert(architectureViolationCount(dependencies) == 1);
    assert(hasArchitectureViolations(dependencies) == true);
    return 0;
}
