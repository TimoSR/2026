#include <memory>
#include <string>

using namespace std;

class IEmailGateway {
public:
    virtual ~IEmailGateway() = default;
    virtual bool sendWelcomeEmail(string userEmail) = 0;
};

class UserRegistrationService {
private:
    shared_ptr<IEmailGateway> emailGateway;

public:
    UserRegistrationService(shared_ptr<IEmailGateway> emailGatewayValue)
        : emailGateway(emailGatewayValue) {
    }

    bool registerUser(string userEmail) {
        if (userEmail.empty()) {
            return false;
        }

        // Business flow depends on abstraction, not concrete implementation.
        return emailGateway->sendWelcomeEmail(userEmail);
    }
};

class FakeEmailGatewayAlwaysSuccess : public IEmailGateway {
public:
    bool sendWelcomeEmail(string userEmail) override {
        (void)userEmail;
        return true;
    }
};

class FakeEmailGatewayAlwaysFail : public IEmailGateway {
public:
    bool sendWelcomeEmail(string userEmail) override {
        (void)userEmail;
        return false;
    }
};

class SpyEmailGateway : public IEmailGateway {
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
    shared_ptr<IEmailGateway> fakeGateway = make_shared<FakeEmailGatewayAlwaysSuccess>();
    UserRegistrationService service(fakeGateway);

    bool didRegister = service.registerUser("nora@example.com");
    cout << "Register result: " << didRegister << "\n";
    return 0;
}
#endif
