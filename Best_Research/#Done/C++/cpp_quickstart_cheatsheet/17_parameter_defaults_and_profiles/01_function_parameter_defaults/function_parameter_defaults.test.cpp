#include <cassert>
using namespace std;

#include "function_parameter_defaults.cpp"

int main() {
    {
        EmailMessage message = buildEmailMessage("nora@example.com", "Welcome");
        assert(message.getFrom() == "noreply@learning.local");
        assert(message.getTrackOpens() == true);
        assert(message.getRetryCount() == 3);
    }

    {
        EmailMessage message = buildEmailMessage("ava@example.com", "Alert", "alerts@learning.local", false, 1);
        assert(message.getFrom() == "alerts@learning.local");
        assert(message.getTrackOpens() == false);
        assert(message.getRetryCount() == 1);
    }

    {
        EmailMessage legacy = buildEmailMessageLegacyPlaceholderStyle("mira@example.com", "Legacy");
        assert(legacy.getFrom().empty() == true);
        assert(legacy.getRetryCount() == 0);
        assert(usesPlaceholderValues(legacy) == true);
    }

    {
        EmailMessage standard = buildEmailMessage("nora@example.com", "Welcome");
        assert(usesPlaceholderValues(standard) == false);
    }

    return 0;
}
