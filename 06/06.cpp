#include <iostream>
#include <fstream>
#include <unordered_map>
#include <sstream>

void part1(std::unordered_map<unsigned long, unsigned long> fish) {

    for(unsigned long i = 0; i < 80; i++) {
        unsigned long timer0 = fish[0];
        unsigned long timer1 = fish[1];
        unsigned long timer2 = fish[2];
        unsigned long timer3 = fish[3];
        unsigned long timer4 = fish[4];
        unsigned long timer5 = fish[5];
        unsigned long timer6 = fish[6];
        unsigned long timer7 = fish[7];
        unsigned long timer8 = fish[8];


        fish[0] = timer1;
        fish[1] = timer2;
        fish[2] = timer3;
        fish[3] = timer4;
        fish[4] = timer5;
        fish[5] = timer6;
        fish[6] = timer0 + timer7;
        fish[7] = timer8;
        fish[8] = timer0;
    }

    unsigned long amount = 0;
    for(int i = 0; i < 9; i++) {
        amount += fish[i];
    }

    std::cout << "Part 1 Output: " << amount << "\n";

}

void part2(std::unordered_map<unsigned long, unsigned long> fish) {

    for(unsigned long i = 0; i < 256; i++) {
        unsigned long timer0 = fish[0];
        unsigned long timer1 = fish[1];
        unsigned long timer2 = fish[2];
        unsigned long timer3 = fish[3];
        unsigned long timer4 = fish[4];
        unsigned long timer5 = fish[5];
        unsigned long timer6 = fish[6];
        unsigned long timer7 = fish[7];
        unsigned long timer8 = fish[8];

        fish[0] = timer1;
        fish[1] = timer2;
        fish[2] = timer3;
        fish[3] = timer4;
        fish[4] = timer5;
        fish[5] = timer6;
        fish[6] = timer0 + timer7;
        fish[7] = timer8;
        fish[8] = timer0;
    }

    unsigned long amount = 0;
    for(int i = 0; i < 9; i++) {
        amount += fish[i];
    }

    std::cout << "Part 2 Output: " << amount << "\n";
}

int main() {

    std::unordered_map<unsigned long, unsigned long> fish;

    std::string input = "5,1,2,1,5,3,1,1,1,1,1,2,5,4,1,1,1,1,2,1,2,1,1,1,1,1,2,1,5,1,1,1,3,1,1,1,3,1,1,3,1,1,4,3,1,1,4,1,1,1,1,2,1,1,1,5,1,1,5,1,1,1,4,4,2,5,1,1,5,1,1,2,2,1,2,1,1,5,3,1,2,1,1,3,1,4,3,3,1,1,3,1,5,1,1,3,1,1,4,4,1,1,1,5,1,1,1,4,4,1,3,1,4,1,1,4,5,1,1,1,4,3,1,4,1,1,4,4,3,5,1,2,2,1,2,2,1,1,1,2,1,1,1,4,1,1,3,1,1,2,1,4,1,1,1,1,1,1,1,1,2,2,1,1,5,5,1,1,1,5,1,1,1,1,5,1,3,2,1,1,5,2,3,1,2,2,2,5,1,1,3,1,1,1,5,1,4,1,1,1,3,2,1,3,3,1,3,1,1,1,1,1,1,1,2,3,1,5,1,4,1,3,5,1,1,1,2,2,1,1,1,1,5,4,1,1,3,1,2,4,2,1,1,3,5,1,1,1,3,1,1,1,5,1,1,1,1,1,3,1,1,1,4,1,1,1,1,2,2,1,1,1,1,5,3,1,2,3,4,1,1,5,1,2,4,2,1,1,1,2,1,1,1,1,1,1,1,4,1,5";
    std::stringstream stream(input);

    std::string data;
    while(getline(stream, data, ',')) {
        unsigned long key = atoi(data.c_str());
        if(fish.find(key) == fish.end()) {
            fish[key] = 0;
        }
        fish[key]++;
    }

    part1(fish);
    part2(fish);

    return 0;
}