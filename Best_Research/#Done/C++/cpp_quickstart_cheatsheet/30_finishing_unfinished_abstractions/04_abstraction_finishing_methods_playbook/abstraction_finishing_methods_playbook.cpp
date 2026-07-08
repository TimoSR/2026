#include <string>
#include <vector>

using namespace std;

enum class AbstractionMethodKind {
    FacadeWrapper,
    ProfileDefaults,
    ErrorNormalization,
    LifecycleManager,
    AdapterBoundary
};

string methodName(AbstractionMethodKind kind) {
    if (kind == AbstractionMethodKind::FacadeWrapper) {
        return "Facade wrapper";
    }
    if (kind == AbstractionMethodKind::ProfileDefaults) {
        return "Profile defaults";
    }
    if (kind == AbstractionMethodKind::ErrorNormalization) {
        return "Error normalization";
    }
    if (kind == AbstractionMethodKind::LifecycleManager) {
        return "Lifecycle manager";
    }
    return "Adapter boundary";
}

string methodIntent(AbstractionMethodKind kind) {
    if (kind == AbstractionMethodKind::FacadeWrapper) {
        return "Hide low-level API details behind domain methods.";
    }
    if (kind == AbstractionMethodKind::ProfileDefaults) {
        return "Preconfigure safe/fast/debug behavior.";
    }
    if (kind == AbstractionMethodKind::ErrorNormalization) {
        return "Convert diverse low-level errors to one app-level model.";
    }
    if (kind == AbstractionMethodKind::LifecycleManager) {
        return "Centralize init/shutdown and ownership via RAII.";
    }
    return "Translate external library types to internal project types.";
}

class AbstractionMethod {
private:
    AbstractionMethodKind kind = AbstractionMethodKind::FacadeWrapper;

public:
    explicit AbstractionMethod(AbstractionMethodKind kindValue) : kind(kindValue) {
    }

    string getName() const {
        return methodName(kind);
    }

    string getIntent() const {
        return methodIntent(kind);
    }
};

class CapabilityNeeds {
private:
    bool hasLongCallChains = false;
    bool hasManyConfigurationFlags = false;
    bool hasManyErrorShapes = false;
    bool hasLifecycleComplexity = false;
    bool exposesForeignTypes = false;

public:
    CapabilityNeeds(
        bool longCallChains,
        bool manyConfigurationFlags,
        bool manyErrorShapes,
        bool lifecycleComplexity,
        bool foreignTypes
    ) : hasLongCallChains(longCallChains),
        hasManyConfigurationFlags(manyConfigurationFlags),
        hasManyErrorShapes(manyErrorShapes),
        hasLifecycleComplexity(lifecycleComplexity),
        exposesForeignTypes(foreignTypes) {
    }

    bool needsFacadeWrapper() const {
        return hasLongCallChains;
    }

    bool needsProfileDefaults() const {
        return hasManyConfigurationFlags;
    }

    bool needsErrorNormalization() const {
        return hasManyErrorShapes;
    }

    bool needsLifecycleManager() const {
        return hasLifecycleComplexity;
    }

    bool needsAdapterBoundary() const {
        return exposesForeignTypes;
    }
};

vector<AbstractionMethod> finishingMethods() {
    vector<AbstractionMethod> methods;
    methods.push_back(AbstractionMethod(AbstractionMethodKind::FacadeWrapper));
    methods.push_back(AbstractionMethod(AbstractionMethodKind::ProfileDefaults));
    methods.push_back(AbstractionMethod(AbstractionMethodKind::ErrorNormalization));
    methods.push_back(AbstractionMethod(AbstractionMethodKind::LifecycleManager));
    methods.push_back(AbstractionMethod(AbstractionMethodKind::AdapterBoundary));
    return methods;
}

bool containsMethod(vector<AbstractionMethodKind> methods, AbstractionMethodKind kind) {
    for (AbstractionMethodKind existingKind : methods) {
        if (existingKind == kind) {
            return true;
        }
    }
    return false;
}

void addMethodIfMissing(vector<AbstractionMethodKind>& methods, AbstractionMethodKind kind) {
    if (!containsMethod(methods, kind)) {
        methods.push_back(kind);
    }
}

vector<AbstractionMethodKind> recommendMethodsFromNeeds(CapabilityNeeds needs) {
    vector<AbstractionMethodKind> picks;

    if (needs.needsFacadeWrapper()) {
        addMethodIfMissing(picks, AbstractionMethodKind::FacadeWrapper);
    }
    if (needs.needsProfileDefaults()) {
        addMethodIfMissing(picks, AbstractionMethodKind::ProfileDefaults);
    }
    if (needs.needsErrorNormalization()) {
        addMethodIfMissing(picks, AbstractionMethodKind::ErrorNormalization);
    }
    if (needs.needsLifecycleManager()) {
        addMethodIfMissing(picks, AbstractionMethodKind::LifecycleManager);
    }
    if (needs.needsAdapterBoundary()) {
        addMethodIfMissing(picks, AbstractionMethodKind::AdapterBoundary);
    }

    if (picks.empty()) {
        addMethodIfMissing(picks, AbstractionMethodKind::FacadeWrapper);
    }

    return picks;
}

CapabilityNeeds needsForTechnology(string technologyName) {
    if (technologyName == "async") {
        return CapabilityNeeds(true, true, true, false, false);
    }

    if (technologyName == "sockets") {
        return CapabilityNeeds(false, false, true, true, true);
    }

    if (technologyName == "multithreading") {
        return CapabilityNeeds(true, true, false, true, false);
    }

    return CapabilityNeeds(true, false, false, false, false);
}

vector<string> recommendedMethodsFor(string technologyName) {
    CapabilityNeeds needs = needsForTechnology(technologyName);
    vector<AbstractionMethodKind> methods = recommendMethodsFromNeeds(needs);
    vector<string> picks;
    for (AbstractionMethodKind kind : methods) {
        picks.push_back(methodName(kind));
    }
    return picks;
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    cout << "[Finishing methods]\n";
    vector<AbstractionMethod> methods = finishingMethods();
    for (AbstractionMethod method : methods) {
        cout << method.getName() << " -> " << method.getIntent() << "\n";
    }

    cout << "\n[Recommended for sockets]\n";
    vector<string> picks = recommendedMethodsFor("sockets");
    for (string pick : picks) {
        cout << pick << "\n";
    }
    return 0;
}
#endif
