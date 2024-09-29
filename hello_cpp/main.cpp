extern "C" void print_char(char c);

void print_string(const char* str) {
    while (*str) {
        print_char(*str);
        str++;
    }
}   

int main() {
    print_string("Hello, World!\n");

    while (1);
    return 0;
}