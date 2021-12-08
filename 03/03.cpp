#include <iostream>
#include <fstream>
#include <vector>
#include <cmath>

struct BitComposition {
    char bits[12];
};

void part1(std::vector<BitComposition>& input) {

    unsigned int gamma_rate = 0;
    unsigned int epsilon_rate = 0;

    for(unsigned int i = 0; i < 12; i++) {

        unsigned int ones = 0;
        unsigned int zeros = 0;

        for(unsigned int j = 0; j < input.size(); j++) {

            if(input[j].bits[i] == 1) {
                ones++;
            } else {
                zeros++;
            }

        }

        if(ones > zeros) {
            gamma_rate += pow(2, 11 - i);
        } else {
            epsilon_rate += pow(2, 11 - i);
        }

    }

    std::cout << "Part 1 Output: " << gamma_rate * epsilon_rate << "\n";
    
}

std::vector<BitComposition> get_oxygen_generator_ratings_from_bit_index(std::vector<BitComposition>& oxygen_generator_ratings, unsigned int index) {

    std::vector<BitComposition> return_vector;

    if(oxygen_generator_ratings.size() > 1) {
        bool is_one_most_common = false;

        unsigned int zeros = 0;
        unsigned int ones = 0;

        for(unsigned int i = 0; i < oxygen_generator_ratings.size(); i++) {
            if(oxygen_generator_ratings[i].bits[index] == 1) {
                ones++;
            } else {
                zeros++;
            }
        }

        is_one_most_common = ones >= zeros;

        for(unsigned int i = 0; i < oxygen_generator_ratings.size(); i++) {
            if(is_one_most_common && oxygen_generator_ratings[i].bits[index] == 1) {
                return_vector.push_back(oxygen_generator_ratings[i]);
            } else if(!is_one_most_common && oxygen_generator_ratings[i].bits[index] == 0) {
                return_vector.push_back(oxygen_generator_ratings[i]);
            }
        }
    } else {
        return_vector = oxygen_generator_ratings;
    }

    return return_vector;

}

std::vector<BitComposition> get_co2_scrubber_ratings_from_bit_index(std::vector<BitComposition>& co2_scrubber_ratings, unsigned int index) {

    std::vector<BitComposition> return_vector;

    if(co2_scrubber_ratings.size() > 1) {
        bool is_one_least_common = false;

        unsigned int zeros = 0;
        unsigned int ones = 0;

        for(unsigned int i = 0; i < co2_scrubber_ratings.size(); i++) {
            if(co2_scrubber_ratings[i].bits[index] == 1) {
                ones++;
            } else {
                zeros++;
            }
        }

        if(ones < zeros) {
            is_one_least_common = true;
        } else if(ones > zeros) {
            is_one_least_common = false;
        } else {
            is_one_least_common = false;
        }

        for(unsigned int i = 0; i < co2_scrubber_ratings.size(); i++) {
            if(is_one_least_common && co2_scrubber_ratings[i].bits[index] == 1) {
                return_vector.push_back(co2_scrubber_ratings[i]);
            } else if(!is_one_least_common && co2_scrubber_ratings[i].bits[index] == 0) {
                return_vector.push_back(co2_scrubber_ratings[i]);
            }
        }
    } else {
        return_vector = co2_scrubber_ratings;
    }

    return return_vector;

}

void part2(std::vector<BitComposition>& input) {

    std::vector<BitComposition> oxygen_generator_ratings = input;
    std::vector<BitComposition> co2_scrubber_ratings = input;

    for(unsigned int i = 0; i < 12; i++) {
        oxygen_generator_ratings = get_oxygen_generator_ratings_from_bit_index(oxygen_generator_ratings, i);
        co2_scrubber_ratings = get_co2_scrubber_ratings_from_bit_index(co2_scrubber_ratings, i);
    }

    unsigned int oxygen_generator_rating = 0;
    unsigned int co2_scrubber_rating = 0;

    for(unsigned int i = 0; i < 12; i++) {
        
        if(oxygen_generator_ratings[0].bits[i] == 1) {
            oxygen_generator_rating += pow(2, 11 - i);
        }

        if(co2_scrubber_ratings[0].bits[i] == 1) {
            co2_scrubber_rating += pow(2, 11 - i);
        }

    }

    std::cout << "Part 1 Output: " << co2_scrubber_rating * oxygen_generator_rating << "\n";
    
}

int main() {

    std::ifstream input("input.txt");
    std::vector<BitComposition> input_data;
    if(input.is_open()) {
        std::string data;
        while(std::getline(input, data)) {
            
            BitComposition bit_composition;

            for(unsigned int i = 0; i < 12; i++) {
                if(data[i] == '1') {
                    bit_composition.bits[i] = 1;
                } else {
                    bit_composition.bits[i] = 0;
                }
            }
            input_data.push_back(bit_composition);

        }
        input.close();
    }

    part1(input_data);
    part2(input_data);

    return 0;
}