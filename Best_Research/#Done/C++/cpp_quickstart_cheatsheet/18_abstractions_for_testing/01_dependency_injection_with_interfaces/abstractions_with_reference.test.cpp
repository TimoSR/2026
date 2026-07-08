#include <cassert>
#include <string>

using namespace std;

#include "abstractions_with_reference.cpp"

int main() {
    {
        FakeEmailGatewaySimple fakeGateway(true);
        UserRegistrationServiceSimple service(fakeGateway);
        bool didRegister = service.registerUser("nora@example.com");
        assert(didRegister == true);
    }

    {
        FakeEmailGatewaySimple fakeGateway(false);
        UserRegistrationServiceSimple service(fakeGateway);
        bool didRegister = service.registerUser("nora@example.com");
        assert(didRegister == false);
    }

    {
        SpyEmailGatewaySimple spyGateway;
        UserRegistrationServiceSimple service(spyGateway);
        bool didRegister = service.registerUser("ava@example.com");
        assert(didRegister == true);
        assert(spyGateway.getSentCount() == 1);
        assert(spyGateway.getLastEmail() == "ava@example.com");
    }

    {
        SpyEmailGatewaySimple spyGateway;
        UserRegistrationServiceSimple service(spyGateway);
        bool didRegister = service.registerUser("");
        assert(didRegister == false);
        assert(spyGateway.getSentCount() == 0);
    }

    return 0;
}
