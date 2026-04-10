#include <memory>
#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

enum class ESaveResult {
    Saved,
    Rejected
};

struct FPlayerProfile {
    string playerId = "";
    int level = 1;
    int totalExperience = 0;
};

class IPlayerProfileStore {
public:
    virtual ~IPlayerProfileStore() = default;
    virtual ESaveResult saveProfile(FPlayerProfile playerProfile) = 0;
    virtual FPlayerProfile loadProfile(string playerId) = 0;
};

class IAuditSink {
public:
    virtual ~IAuditSink() = default;
    virtual void writeEntry(string message) = 0;
};

class FInMemoryPlayerProfileStore : public IPlayerProfileStore {
private:
    unordered_map<string, FPlayerProfile> profilesById;

public:
    ESaveResult saveProfile(FPlayerProfile playerProfile) override {
        if (playerProfile.playerId.empty()) {
            return ESaveResult::Rejected;
        }
        profilesById[playerProfile.playerId] = playerProfile;
        return ESaveResult::Saved;
    }

    FPlayerProfile loadProfile(string playerId) override {
        if (profilesById.count(playerId) == 0) {
            FPlayerProfile emptyProfile;
            emptyProfile.playerId = playerId;
            return emptyProfile;
        }
        return profilesById[playerId];
    }
};

class FInMemoryAuditSink : public IAuditSink {
private:
    vector<string> entries;

public:
    void writeEntry(string message) override {
        entries.push_back(message);
    }

    int count() {
        return static_cast<int>(entries.size());
    }

    string latest() {
        if (entries.empty()) {
            return "";
        }
        return entries.back();
    }
};

class FPlayerProgressService {
private:
    shared_ptr<IPlayerProfileStore> playerProfileStore;
    shared_ptr<IAuditSink> auditSink;

public:
    FPlayerProgressService(shared_ptr<IPlayerProfileStore> playerProfileStoreValue, shared_ptr<IAuditSink> auditSinkValue)
        : playerProfileStore(playerProfileStoreValue),
          auditSink(auditSinkValue) {
    }

    bool grantExperience(string playerId, int gainedExperience) {
        if (playerId.empty() || gainedExperience <= 0) {
            auditSink->writeEntry("grantExperience rejected");
            return false;
        }

        FPlayerProfile profile = playerProfileStore->loadProfile(playerId);
        profile.playerId = playerId;
        profile.totalExperience += gainedExperience;
        profile.level = 1 + profile.totalExperience / 100;

        ESaveResult saveResult = playerProfileStore->saveProfile(profile);
        if (saveResult == ESaveResult::Saved) {
            auditSink->writeEntry("grantExperience saved for " + playerId);
            return true;
        }

        auditSink->writeEntry("grantExperience failed for " + playerId);
        return false;
    }

    FPlayerProfile getProfile(string playerId) {
        return playerProfileStore->loadProfile(playerId);
    }
};

#ifdef RUN_DEMO
#include <iostream>

int main() {
    shared_ptr<IPlayerProfileStore> playerProfileStore = make_shared<FInMemoryPlayerProfileStore>();
    shared_ptr<IAuditSink> auditSink = make_shared<FInMemoryAuditSink>();

    FPlayerProgressService playerProgressService(playerProfileStore, auditSink);

    bool firstGrant = playerProgressService.grantExperience("nora", 120);
    bool secondGrant = playerProgressService.grantExperience("nora", 35);
    FPlayerProfile noraProfile = playerProgressService.getProfile("nora");

    cout << "[method] grantExperience\n";
    cout << "first grant ok: " << firstGrant << "\n";
    cout << "second grant ok: " << secondGrant << "\n";
    cout << "nora level: " << noraProfile.level << "\n";
    cout << "nora xp: " << noraProfile.totalExperience << "\n";
    return 0;
}
#endif
