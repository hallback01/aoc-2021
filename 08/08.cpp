#include <iostream>
#include <fstream>
#include <vector>
#include <map>
#include <algorithm>

std::vector<std::string> split_string(std::string string, std::string delimiter) {
    std::vector<std::string> output;
    int position = string.find(delimiter);
    for(size_t position = string.find(delimiter); position != std::string::npos; position = string.find(delimiter)) {
        output.push_back(string.substr(0, position));
        string.erase(0, position + delimiter.length());
    }
    output.push_back(string);
    return output;
}

void part1() {

    unsigned int total = 0;

    std::ifstream input("input.txt");
    if(input.is_open()) {
        std::string data;
        while(std::getline(input, data)) {
            
            std::vector<std::string> sides = split_string(data, " | ");

            std::vector<std::string> right_side = split_string(sides[1], " ");

            for(unsigned int i = 0; i < right_side.size(); i++) {
                
                unsigned int length = right_side[i].length();

                if(length == 2 || length == 4 || length == 7 || length == 3) {
                    total++;
                }

            }

        }
        input.close();
    }

    std::cout << "Part 1 Output: " << total << "\n";

}

unsigned int compare(std::string a, std::string b) {

    unsigned int total = 0;

    for(unsigned int _a = 0; _a < a.length(); _a++) {

        for(unsigned int _b = 0; _b < b.length(); _b++) {

            if(a[_a] == b[_b]) {
                total++;
            }
        }
    }
    return total;
}

std::map<std::string, char> decode_left_side(std::vector<std::string> digit_list) {

    std::map<std::string, char> digits;
    std::map<char, std::string> reverse_digits;

    for(unsigned int i = 0; i < digit_list.size(); i++) {

        if(digit_list[i].length() == 2) {
            reverse_digits.insert({'1', digit_list[i]});
        }

        if(digit_list[i].length() == 4) {
            reverse_digits.insert({'4', digit_list[i]});
        }

        if(digit_list[i].length() == 3) {
            reverse_digits.insert({'7', digit_list[i]});
        }

        if(digit_list[i].length() == 7) {
            reverse_digits.insert({'8', digit_list[i]});
        }

    }

    for(unsigned int i = 0; i < digit_list.size(); i++) {
        if(digit_list[i].length() == 5) { //2, 3, and 5

            if(compare(digit_list[i], reverse_digits['1']) == 1 && compare(digit_list[i], reverse_digits['7']) == 2 && compare(digit_list[i], reverse_digits['4']) == 2) {
                reverse_digits.insert({'2', digit_list[i]});
            }

            if(compare(digit_list[i], reverse_digits['1']) == 2 && compare(digit_list[i], reverse_digits['7']) == 3 && compare(digit_list[i], reverse_digits['4']) == 3) {
                reverse_digits.insert({'3', digit_list[i]});
            }

            if(compare(digit_list[i], reverse_digits['1']) == 1 && compare(digit_list[i], reverse_digits['7']) == 2 && compare(digit_list[i], reverse_digits['4']) == 3) {
                reverse_digits.insert({'5', digit_list[i]});
            }

        }
    }

    for(unsigned int i = 0; i < digit_list.size(); i++) {
        if(digit_list[i].length() == 6) { // 0, 6, 9

            if(compare(digit_list[i], reverse_digits['2']) == 4 && compare(digit_list[i], reverse_digits['3']) == 4 && compare(digit_list[i], reverse_digits['4']) == 3 && compare(digit_list[i], reverse_digits['5']) == 4) {
                reverse_digits.insert({'0', digit_list[i]});
            }

            if(compare(digit_list[i], reverse_digits['2']) == 4 && compare(digit_list[i], reverse_digits['3']) == 4 && compare(digit_list[i], reverse_digits['4']) == 3 && compare(digit_list[i], reverse_digits['5']) == 5) {
                reverse_digits.insert({'6', digit_list[i]});
            }

            if(compare(digit_list[i], reverse_digits['2']) == 4 && compare(digit_list[i], reverse_digits['3']) == 5 && compare(digit_list[i], reverse_digits['4']) == 4 && compare(digit_list[i], reverse_digits['5']) == 5) {
                reverse_digits.insert({'9', digit_list[i]});
            }

        }
    }

    std::string n0 = reverse_digits['0']; std::sort(n0.begin(), n0.end()); digits.insert({n0, '0'});
    std::string n1 = reverse_digits['1']; std::sort(n1.begin(), n1.end()); digits.insert({n1, '1'});
    std::string n2 = reverse_digits['2']; std::sort(n2.begin(), n2.end()); digits.insert({n2, '2'});
    std::string n3 = reverse_digits['3']; std::sort(n3.begin(), n3.end()); digits.insert({n3, '3'});
    std::string n4 = reverse_digits['4']; std::sort(n4.begin(), n4.end()); digits.insert({n4, '4'});
    std::string n5 = reverse_digits['5']; std::sort(n5.begin(), n5.end()); digits.insert({n5, '5'});
    std::string n6 = reverse_digits['6']; std::sort(n6.begin(), n6.end()); digits.insert({n6, '6'});
    std::string n7 = reverse_digits['7']; std::sort(n7.begin(), n7.end()); digits.insert({n7, '7'});
    std::string n8 = reverse_digits['8']; std::sort(n8.begin(), n8.end()); digits.insert({n8, '8'});
    std::string n9 = reverse_digits['9']; std::sort(n9.begin(), n9.end()); digits.insert({n9, '9'});

    return digits;

}

char get_digit(std::string signal_pattern, std::map<std::string, char>& digits) {

    std::sort(signal_pattern.begin(), signal_pattern.end());

    return digits[signal_pattern];

}

void part2() {
    unsigned int total = 0;

    std::ifstream input("input.txt");
    if(input.is_open()) {
        std::string data;

        while(std::getline(input, data)) {
            
            std::vector<std::string> sides = split_string(data, " | ");

            std::vector<std::string> left_side = split_string(sides[0], " ");
            std::vector<std::string> right_side = split_string(sides[1], " ");

            std::map<std::string, char> digits = decode_left_side(left_side);

            std::string digit = "";
            for(unsigned int i = 0; i < right_side.size(); i++) {
                digit += get_digit(right_side[i], digits);
            }
            unsigned int val = atoi(digit.c_str());
            total+=val;

        }
        input.close();
    }

    std::cout << "Part 2 Output: " << total << "\n";
}

int main() {

    part1();
    part2();

    return 0;
}