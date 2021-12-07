#include <iostream>
#include <fstream>
#include <vector>

enum MovementType {
    Forward,
    Down,
    Up
};

struct Command {
    MovementType type;
    unsigned int amount;
};

void part1(std::vector<Command>& input) {

    int depth = 0;
    int horizontal_position = 0;
    int aim = 0;

    for(unsigned int i = 0; i < input.size(); i++) {

        switch(input[i].type){

            case MovementType::Forward: {
                horizontal_position += input[i].amount;
                break;
            }

            case MovementType::Down: {
                depth += input[i].amount;
                break;
            }

            case MovementType::Up: {
                depth -= input[i].amount;
                break;
            }

        }

    }

    std::cout << "Part 1 Output: " << depth * horizontal_position << "\n";

}

void part2(std::vector<Command>& input) {

    int depth = 0;
    int horizontal_position = 0;
    int aim = 0;

    for(unsigned int i = 0; i < input.size(); i++) {

        switch(input[i].type){

            case MovementType::Forward: {
                horizontal_position += input[i].amount;
                depth += aim * input[i].amount;
                break;
            }

            case MovementType::Down: {
                aim += input[i].amount;
                break;
            }

            case MovementType::Up: {
                aim -= input[i].amount;
                break;
            }

        }

    }

    std::cout << "Part 2 Output: " << depth * horizontal_position << "\n";
}

int main() {

    std::ifstream input("input.txt");
    std::vector<Command> input_data;

    if(input.is_open()) {
        std::string data;
        while(std::getline(input, data)) {
            
            Command command;
            unsigned int space = data.find(' ');
            std::string type = data.substr(0, space);
            std::string amount = data.substr(space+1, data.length());
    
            command.amount = atoi(amount.c_str());
            if(type == "forward") {
                command.type = MovementType::Forward;
            } else if(type == "down") {
                command.type = MovementType::Down;
            } else if(type == "up") {
                command.type = MovementType::Up;
            }
            input_data.push_back(command);
        }
        input.close();
    }

    part1(input_data);
    part2(input_data);

    return 0;
}