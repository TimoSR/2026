#include <string>

using namespace std;

class EmailMessage {
private:
    string from = "";
    string to = "";
    string subject = "";
    bool trackOpens = false;
    int retryCount = 0;

public:
    EmailMessage(string fromValue, string toValue, string subjectValue, bool trackOpensValue, int retryCountValue)
        : from(fromValue), to(toValue), subject(subjectValue), trackOpens(trackOpensValue), retryCount(retryCountValue) {
    }

    string getFrom() const {
        return from;
    }

    string getTo() const {
        return to;
    }

    string getSubject() const {
        return subject;
    }

    bool getTrackOpens() const {
        return trackOpens;
    }

    int getRetryCount() const {
        return retryCount;
    }
};

EmailMessage buildEmailMessage(
    string to,
    string subject,
    string from = "noreply@learning.local",
    bool trackOpens = true,
    int retryCount = 3
) {
    // Caller can pass only required business inputs: to + subject.
    // Defaults provide safe and practical behavior.
    return EmailMessage(from, to, subject, trackOpens, retryCount);
}

EmailMessage buildEmailMessageLegacyPlaceholderStyle(string to, string subject) {
    // Common legacy anti-pattern: meaningless placeholders at call-site.
    return EmailMessage("", to, subject, false, 0);
}

bool usesPlaceholderValues(EmailMessage message) {
    return message.getFrom().empty() || message.getRetryCount() == 0;
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    EmailMessage standard = buildEmailMessage("nora@example.com", "Welcome!");
    cout << "[Default call]\n";
    cout << "from: " << standard.getFrom() << "\n";
    cout << "trackOpens: " << standard.getTrackOpens() << "\n";
    cout << "retryCount: " << standard.getRetryCount() << "\n\n";

    EmailMessage custom = buildEmailMessage("ava@example.com", "Alert", "alerts@learning.local", false, 1);
    cout << "[Custom call]\n";
    cout << "from: " << custom.getFrom() << "\n";
    cout << "trackOpens: " << custom.getTrackOpens() << "\n";
    cout << "retryCount: " << custom.getRetryCount() << "\n";

    EmailMessage legacy = buildEmailMessageLegacyPlaceholderStyle("leo@example.com", "Legacy");
    cout << "\n[Legacy placeholder call]\n";
    cout << "from: '" << legacy.getFrom() << "'\n";
    cout << "uses placeholder values: " << usesPlaceholderValues(legacy) << "\n";
    return 0;
}
#endif
