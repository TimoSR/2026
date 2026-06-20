#include <string>

using namespace std;

#include "bad_header_example.h"
#include "good_header_example.h"

int parse(string text) {
    return static_cast<int>(text.size());
}

namespace learning_headers {
    int parseTextLength(std::string text) {
        return static_cast<int>(text.size());
    }
}

namespace analytics_headers {
    int parse(string text) {
        // Different semantic parse: count comma-separated tokens.
        if (text.empty()) {
            return 0;
        }

        int tokenCount = 1;
        for (char character : text) {
            if (character == ',') {
                tokenCount += 1;
            }
        }
        return tokenCount;
    }
}

int parseWithGoodHeaderFunction(string text) {
    return learning_headers::parseTextLength(text);
}

int parseWithAnalyticsNamespace(string text) {
    return analytics_headers::parse(text);
}

bool explicitNamespaceChangesBehavior(string text) {
    int globalParseResult = parse(text);
    int analyticsParseResult = parseWithAnalyticsNamespace(text);
    return globalParseResult != analyticsParseResult;
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    cout << "[Header namespace impact]\n";
    cout << "bad header parse('abc'): " << parse("abc") << "\n";
    cout << "good header parseTextLength('abc'): " << parseWithGoodHeaderFunction("abc") << "\n\n";
    cout << "analytics parse('a,b,c'): " << parseWithAnalyticsNamespace("a,b,c") << "\n";
    cout << "explicit namespace changes behavior: " << explicitNamespaceChangesBehavior("a,b,c") << "\n";
    return 0;
}
#endif
