#include <cassert>
#include <memory>

using namespace std;

#include "module_boundaries_and_ownership.cpp"

int main() {
    shared_ptr<IPlayerProfileStore> playerProfileStore = make_shared<FInMemoryPlayerProfileStore>();
    shared_ptr<FInMemoryAuditSink> inMemoryAuditSink = make_shared<FInMemoryAuditSink>();

    FPlayerProgressService playerProgressService(playerProfileStore, inMemoryAuditSink);

    bool didGrant = playerProgressService.grantExperience("nora", 250);
    assert(didGrant == true);

    FPlayerProfile noraProfile = playerProgressService.getProfile("nora");
    assert(noraProfile.level == 3);
    assert(noraProfile.totalExperience == 250);

    bool rejectedGrant = playerProgressService.grantExperience("", 10);
    assert(rejectedGrant == false);

    assert(inMemoryAuditSink->count() == 2);
    assert(inMemoryAuditSink->latest().find("rejected") != string::npos);
    return 0;
}
