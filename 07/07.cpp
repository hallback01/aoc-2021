#include <iostream>
#include <fstream>
#include <vector>
#include <sstream>

void part1(std::vector<unsigned int>& input, unsigned int max_horizontal_position) {

    unsigned int cheapest = -1;

    for(unsigned int i = 0; i < max_horizontal_position; i++) {

        unsigned int cost = 0;
        unsigned int align = i;

        for(unsigned int pos = 0; pos < input.size(); pos++) {

            cost += abs(input[pos] - align);

        }

        if(cost < cheapest) {
            cheapest = cost;
        }

    }

    std::cout << "Part 1 Output: " << cheapest << "\n";

}

unsigned int new_cost(unsigned int steps) {

    unsigned int cost = (steps * (steps + 1)) / 2;

    return cost;

}

void part2(std::vector<unsigned int>& input, unsigned int max_horizontal_position) {

    unsigned int cheapest = -1;

    for(unsigned int i = 0; i < max_horizontal_position; i++) {

        unsigned int cost = 0;
        unsigned int align = i;

        for(unsigned int pos = 0; pos < input.size(); pos++) {

            cost += new_cost(abs(input[pos] - align));

        }

        if(cost < cheapest) {
            cheapest = cost;
        }

    }

    std::cout << "Part 2 Output: " << cheapest << "\n";
}

int main() {

    std::ifstream input("input.txt");
    std::vector<unsigned int> horizontal_positions;

    std::string data;

    if(input.is_open()) {
        std::ostringstream stream;
        stream << input.rdbuf();
        data = stream.str();

        input.close();
    }

    std::stringstream stream(data);

    std::string horizontal_position;

    unsigned int highest_horizontal_position = 0;

    while(getline(stream, horizontal_position, ',')) {
        unsigned int value = atoi(horizontal_position.c_str());
        horizontal_positions.push_back(value);
        if(value > highest_horizontal_position) {
            highest_horizontal_position = value;
        }
    }

    part1(horizontal_positions, highest_horizontal_position);
    part2(horizontal_positions, highest_horizontal_position);

    return 0;
}