#include <iostream>

int main(void) {
    const auto x = 0x123;
    std::cout << x << std::endl;
    const auto str = to_string(123);
    return 0;
}