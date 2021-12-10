#include <iostream>
#include <fstream>
#include <vector>
#include <stack>
#include <algorithm>

unsigned int score_table(char character) {

    switch(character) {
        case ')': return 3;
        case ']': return 57;
        case '}': return 1197;
        case '>': return 25137;
    }
    return 0;
}

bool is_corrupted(char input, unsigned int* output_score, std::stack<char>& open_bracket_stack) {
    if(input == '{' || input == '[' || input == '<' || input == '(') {
        open_bracket_stack.push(input);
    } else {
        char top = open_bracket_stack.top(); open_bracket_stack.pop();
        if(top == '{' && input != '}') {
            if(output_score) {
                *output_score += score_table(input);
            }    
            return true;   
        }
        if(top == '[' && input != ']') {
            if(output_score) {
                *output_score += score_table(input);
            } 
            return true;   
        }
        if(top == '<' && input != '>') {
            if(output_score) {
                *output_score += score_table(input);
            } 
            return true;   
        }
        if(top == '(' && input != ')') {
            if(output_score) {
                *output_score += score_table(input);
            } 
            return true;   
        }
    }
    return false;
}

void part1(std::vector<std::string>& input) {

    unsigned int score = 0;
    for(unsigned int i = 0; i < input.size(); i++) {
        std::stack<char> open_bracket_stack;
        for(unsigned int j = 0; j < input[i].length(); j++) {
            if(is_corrupted(input[i][j], &score, open_bracket_stack)) {
                break;
            }
        }
    }

    std::cout << "Part 1 Output: " << score << "\n";

}

size_t get_incomplete_score(std::stack<char>& stack) {

    std::vector<char> reverse;
    size_t score = 0;
    while(!stack.empty()) {
        char top = stack.top(); stack.pop();
        if(top == '{') {
            score *= 5;
            score += 3;
        }
        if(top == '[') {
            score *= 5;
            score += 2;
        }
        if(top == '<') {
            score *= 5;
            score += 4;
        }
        if(top == '(') {
            score *= 5;
            score += 1;
        }
    }
    return score;
}

void part2(std::vector<std::string>& input) {

    std::vector<size_t> score;
    for(unsigned int i = 0; i < input.size(); i++) {
        std::stack<char> open_bracket_stack;

        bool corrupted = false;

        for(unsigned int j = 0; j < input[i].length(); j++) {
            corrupted = is_corrupted(input[i][j], nullptr, open_bracket_stack);
            if(corrupted) {
                break;
            }
        }

        if(!corrupted) {

            if(open_bracket_stack.size() > 0) {
                score.push_back(get_incomplete_score(open_bracket_stack));
            }

        }
    }

    std::sort(score.begin(), score.end());
    size_t final_score = score[score.size() / 2];
    std::cout << "Part 2 Output: " << final_score << "\n";
}

int main() {

    std::ifstream input("input.txt");
    std::vector<std::string> input_data;
    if(input.is_open()) {
        std::string data;
        while(std::getline(input, data)) {
            input_data.push_back(data);
        }
        input.close();
    }

    part1(input_data);
    part2(input_data);

    return 0;
}