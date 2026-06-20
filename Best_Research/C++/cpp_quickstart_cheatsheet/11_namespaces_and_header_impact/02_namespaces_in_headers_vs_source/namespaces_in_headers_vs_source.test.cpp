#include <cassert>
using namespace std;

#include "namespaces_in_headers_vs_source.cpp"

int main() {
    assert(parse("abcd") == 4);
    assert(parseWithGoodHeaderFunction("hello") == 5);
    assert(parseWithAnalyticsNamespace("a,b,c") == 3);
    assert(explicitNamespaceChangesBehavior("a,b,c") == true);
    assert(explicitNamespaceChangesBehavior("x") == false);
    return 0;
}
