#include <cassert>
#include <memory>
#include <string>

using namespace std;

#include "unreal_style_prefix_conventions.cpp"

int main() {
    APlayerCharacter playerCharacter("Nora", 100);
    playerCharacter.pickUpItem("Key");

    FDamageEvent damageEvent;
    damageEvent.damageAmount = 25;
    damageEvent.damageType = "Ice";
    playerCharacter.applyDamage(damageEvent);

    assert(playerCharacter.getPlayerName() == "Nora");
    assert(playerCharacter.getHealthPoints() == 75);
    assert(playerCharacter.getInventoryComponent()->hasItem("Key") == true);
    assert(playerCharacter.isAlive() == true);

    shared_ptr<IInteractable> interactableInventory = playerCharacter.getInventoryComponent();
    assert(interactableInventory->interact("Nora") == EInteractionResult::Success);

    TObjectRegistry<string> registry;
    registry.add("APlayerCharacter");
    registry.add("UInventoryComponent");
    assert(registry.count() == 2);

    assert(hasUnrealStylePrefix("APlayerCharacter", 'A') == true);
    assert(hasUnrealStylePrefix("IInteractable", 'I') == true);
    assert(prefixMeaning('F').find("Struct") != string::npos);
    return 0;
}
