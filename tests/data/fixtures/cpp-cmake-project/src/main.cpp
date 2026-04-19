#include "math.h"

int main() {
    // Intentional error: undeclared variable
    int result = undefined_var + 1;
    
    // Intentional warning: unused variable
    int unused_var = 42;
    
    return add(result, 1);
}
