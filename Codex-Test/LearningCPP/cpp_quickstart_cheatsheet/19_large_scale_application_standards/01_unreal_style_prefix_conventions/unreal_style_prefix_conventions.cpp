#include <memory>
#include <string>
#include <vector>

using namespace std;

enum class EInteractionResult {
    Success,
    NoEffect
};

struct FDamageEvent {
    int damageAmount = 0;
    string damageType = "";
};

class IInteractable {
public:
    virtual ~IInteractable() = default;
    virtual string getDisplayName() = 0;
    virtual EInteractionResult interact(string actorName) = 0;
};

class UInventoryComponent : public IInteractable {
private:
    string ownerName;
    vector<string> itemNames;

public:
    UInventoryComponent(string ownerNameValue)
        : ownerName(ownerNameValue) {
    }

    void addItem(string itemName) {
        itemNames.push_back(itemName);
    }

    bool hasItem(string itemName) {
        for (string currentItemName : itemNames) {
            if (currentItemName == itemName) {
                return true;
            }
        }
        return false;
    }

    int itemCount() {
        return static_cast<int>(itemNames.size());
    }

    string getDisplayName() override {
        return "UInventoryComponent(" + ownerName + ")";
    }

    EInteractionResult interact(string actorName) override {
        if (actorName.empty()) {
            return EInteractionResult::NoEffect;
        }
        return EInteractionResult::Success;
    }
};

class APlayerCharacter {
private:
    string playerName;
    int healthPoints;
    shared_ptr<UInventoryComponent> inventoryComponent;

public:
    APlayerCharacter(string playerNameValue, int healthPointsValue)
        : playerName(playerNameValue),
          healthPoints(healthPointsValue),
          inventoryComponent(make_shared<UInventoryComponent>(playerNameValue)) {
    }

    string getPlayerName() {
        return playerName;
    }

    int getHealthPoints() {
        return healthPoints;
    }

    shared_ptr<UInventoryComponent> getInventoryComponent() {
        return inventoryComponent;
    }

    void pickUpItem(string itemName) {
        inventoryComponent->addItem(itemName);
    }

    void applyDamage(FDamageEvent damageEvent) {
        healthPoints -= damageEvent.damageAmount;
        if (healthPoints < 0) {
            healthPoints = 0;
        }
    }

    bool isAlive() {
        return healthPoints > 0;
    }
};

template <typename TItem>
class TObjectRegistry {
private:
    vector<TItem> items;

public:
    void add(TItem item) {
        items.push_back(item);
    }

    int count() {
        return static_cast<int>(items.size());
    }
};

bool hasUnrealStylePrefix(string typeName, char prefix) {
    if (typeName.empty()) {
        return false;
    }
    return typeName[0] == prefix;
}

string prefixMeaning(char prefix) {
    if (prefix == 'A') {
        return "A = Actor-like world object";
    }
    if (prefix == 'U') {
        return "U = UObject-like managed object";
    }
    if (prefix == 'I') {
        return "I = Interface";
    }
    if (prefix == 'F') {
        return "F = Struct/value type";
    }
    if (prefix == 'E') {
        return "E = Enum type";
    }
    if (prefix == 'T') {
        return "T = Template type";
    }
    return "Unknown prefix";
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    cout << "[Unreal-style prefix demo]\n";
    cout << prefixMeaning('A') << "\n";
    cout << prefixMeaning('U') << "\n";
    cout << prefixMeaning('I') << "\n";
    cout << prefixMeaning('F') << "\n";
    cout << prefixMeaning('E') << "\n";
    cout << prefixMeaning('T') << "\n\n";

    APlayerCharacter playerCharacter("Nora", 120);
    playerCharacter.pickUpItem("Keycard");

    FDamageEvent fireDamage;
    fireDamage.damageAmount = 35;
    fireDamage.damageType = "Fire";
    playerCharacter.applyDamage(fireDamage);

    shared_ptr<IInteractable> interactableInventory = playerCharacter.getInventoryComponent();
    EInteractionResult interactionResult = interactableInventory->interact("Nora");

    TObjectRegistry<string> debugTypeRegistry;
    debugTypeRegistry.add("APlayerCharacter");
    debugTypeRegistry.add("UInventoryComponent");

    cout << "Player: " << playerCharacter.getPlayerName() << "\n";
    cout << "Health after damage: " << playerCharacter.getHealthPoints() << "\n";
    cout << "Inventory items: " << playerCharacter.getInventoryComponent()->itemCount() << "\n";
    cout << "Interaction success: " << (interactionResult == EInteractionResult::Success) << "\n";
    cout << "Registry count: " << debugTypeRegistry.count() << "\n";
    return 0;
}
#endif
