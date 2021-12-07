#include <iostream>
#include <fstream>
#include <vector>

void part1(std::vector<unsigned int>& input) {

    unsigned int previous_value = 0;
    unsigned int increments = 0;

    for(unsigned int i = 0; i < input.size(); i++) {
        if(input[i] > previous_value) {
            increments++;
        }
        previous_value = input[i];
    }

    std::cout << "Part 1 Output: "  << (increments - 1) << "\n";
}

void part2(std::vector<unsigned int>& input) {

    std::vector<unsigned int> threepair_input;

    unsigned int previous_value = 0;
    unsigned int increments = 0;

    for(int i = 0; i < input.size() - 2; i++) {

        unsigned int value = input[i] + input[i+1] + input[i+2];
        if(value > previous_value) {
            increments++;
        }
        previous_value = value;

        std::cout << value << "\n";

    }

    std::cout << "Part 2 Output: " << (increments - 1) << "\n";

}

int main() {

    std::ifstream input("input.txt");
    std::vector<unsigned int> input_data;
    if(input.is_open()) {
        std::string data;
        while(std::getline(input, data)) {
            input_data.push_back(atoi(data.c_str()));
        }
        input.close();
    }

    part1(input_data);
    part2(input_data);

    return 0;
}