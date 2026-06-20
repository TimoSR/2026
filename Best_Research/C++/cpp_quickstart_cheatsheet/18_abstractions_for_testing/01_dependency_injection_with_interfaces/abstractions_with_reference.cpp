#include <string>

using namespace std;

class IEmailGatewaySimple {
public:
    virtual ~IEmailGatewaySimple() = default;
    virtual bool sendWelcomeEmail(string userEmail) = 0;
};

class UserRegistrationServiceSimple {
private:
    IEmailGatewaySimple& emailGateway;

public:
    UserRegistrationServiceSimple(IEmailGatewaySimple& emailGatewayValue)
        : emailGateway(emailGatewayValue) {
    }

    bool registerUser(string userEmail) {
        if (userEmail.empty()) {
            return false;
        }
        return emailGateway.sendWelcomeEmail(userEmail);
    }
};

class FakeEmailGatewaySimple : public IEmailGatewaySimple {
private:
    bool shouldSucceed;

public:
    FakeEmailGatewaySimple(bool shouldSucceedValue) : shouldSucceed(shouldSucceedValue) {
    }

    bool sendWelcomeEmail(string userEmail) override {
        (void)userEmail;
        return shouldSucceed;
    }
};

class SpyEmailGatewaySimple : public IEmailGatewaySimple {
private:
    int sentCount = 0;
    string lastEmail = "";

public:
    bool sendWelcomeEmail(string userEmail) override {
        sentCount += 1;
        lastEmail = userEmail;
        return true;
    }

    int getSentCount() const {
        return sentCount;
    }

    string getLastEmail() const {
        return lastEmail;
    }
};

#ifdef RUN_DEMO
#include <iostream>

int main() {
    FakeEmailGatewaySimple fakeGateway(true);
    UserRegistrationServiceSimple service(fakeGateway);
    bool didRegister = service.registerUser("nora@example.com");
    cout << "Register result (simplified): " << didRegister << "\n";
    return 0;
}
#endif
