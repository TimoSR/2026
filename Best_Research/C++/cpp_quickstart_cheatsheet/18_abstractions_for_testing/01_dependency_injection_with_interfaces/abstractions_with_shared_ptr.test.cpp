#include <cassert>
#include <memory>
#include <string>

using namespace std;

#include "abstractions_with_shared_ptr.cpp"

int main() {
    {
        shared_ptr<IEmailGateway> fakeGateway = make_shared<FakeEmailGatewayAlwaysSuccess>();
        UserRegistrationService service(fakeGateway);

        bool didRegister = service.registerUser("nora@example.com");
        assert(didRegister == true);
    }

    {
        shared_ptr<IEmailGateway> fakeGateway = make_shared<FakeEmailGatewayAlwaysFail>();
        UserRegistrationService service(fakeGateway);

        bool didRegister = service.registerUser("nora@example.com");
        assert(didRegister == false);
    }

    {
        shared_ptr<SpyEmailGateway> spyGateway = make_shared<SpyEmailGateway>();
        UserRegistrationService service(spyGateway);

        bool didRegister = service.registerUser("ava@example.com");
        assert(didRegister == true);
        assert(spyGateway->getSentCount() == 1);
        assert(spyGateway->getLastEmail() == "ava@example.com");
    }

    {
        shared_ptr<SpyEmailGateway> spyGateway = make_shared<SpyEmailGateway>();
        UserRegistrationService service(spyGateway);

        bool didRegister = service.registerUser("");
        assert(didRegister == false);
        assert(spyGateway->getSentCount() == 0);
    }

    return 0;
}
