#include <iostream>
#include <fstream>
#include <vector>
#include <sstream>

struct Number {
    bool drawn = false;
    unsigned short number;
};

struct Board {
    Number grid[5][5];
};

bool board_has_bingo(unsigned int& out_bingo_number, unsigned int* out_board_index, std::vector<Board>& boards) {

    for(unsigned int i = 0; i < boards.size(); i++) {

        for(unsigned int vertical = 0; vertical < 5; vertical++) {
            if(boards[i].grid[0][vertical].drawn && boards[i].grid[1][vertical].drawn && boards[i].grid[2][vertical].drawn && boards[i].grid[3][vertical].drawn && boards[i].grid[4][vertical].drawn) {
                out_bingo_number = 0;
                for(unsigned short x = 0; x < 5; x++) {
                    for(unsigned short y = 0; y < 5; y++) {
                        if(!boards[i].grid[x][y].drawn) {
                            out_bingo_number += boards[i].grid[x][y].number;
                        }
                    }
                }
                if(out_board_index) {
                    *out_board_index = i;
                }
                return true;
            }
        }

        for(unsigned int horizontal = 0; horizontal < 5; horizontal++) {
            if(boards[i].grid[horizontal][0].drawn && boards[i].grid[horizontal][1].drawn && boards[i].grid[horizontal][2].drawn && boards[i].grid[horizontal][3].drawn && boards[i].grid[horizontal][4].drawn) {
                out_bingo_number = 0;
                for(unsigned short x = 0; x < 5; x++) {
                    for(unsigned short y = 0; y < 5; y++) {
                        if(!boards[i].grid[x][y].drawn) {
                            out_bingo_number += boards[i].grid[x][y].number;
                        }
                    }
                }
                if(out_board_index) {
                    *out_board_index = i;
                }
                return true;
            }
        }

    }

    return false;

}

void part1(std::vector<Board>& boards, std::vector<unsigned int>& number_sequence) {

    unsigned int number;

    unsigned int bingo_number_index = 0;

    while(!board_has_bingo(number, nullptr, boards)) {

        for(unsigned int i = 0; i < boards.size(); i++) {
            for(unsigned short x = 0; x < 5; x++) {
                for(unsigned short y = 0; y < 5; y++) {
                    if(boards[i].grid[x][y].number == number_sequence[bingo_number_index]) {
                        boards[i].grid[x][y].drawn = true;
                    }
                }
            }
        }

        bingo_number_index++;
    }

    std::cout << "Part 1 Output: " << number * number_sequence[bingo_number_index-1] << "\n";

}

void part2(std::vector<Board> boards, std::vector<unsigned int>& number_sequence) {

    unsigned int number;

    unsigned int board_index = 0;

    unsigned int bingo_number_index = 0;

    while(boards.size() > 1) {

        while(board_has_bingo(number, &board_index, boards)) {
            boards.erase(boards.begin() + board_index);
        }

        for(unsigned int i = 0; i < boards.size(); i++) {
            for(unsigned short x = 0; x < 5; x++) {
                for(unsigned short y = 0; y < 5; y++) {
                    if(boards[i].grid[x][y].number == number_sequence[bingo_number_index]) {
                        boards[i].grid[x][y].drawn = true;
                    }
                }
            }
        }

        bingo_number_index++;

    }

    while(!board_has_bingo(number, nullptr, boards)) {
        for(unsigned int i = 0; i < boards.size(); i++) {
            for(unsigned short x = 0; x < 5; x++) {
                for(unsigned short y = 0; y < 5; y++) {
                    if(boards[i].grid[x][y].number == number_sequence[bingo_number_index]) {
                        boards[i].grid[x][y].drawn = true;
                    }
                }
            }
        }

        bingo_number_index++;
    }

    std::cout << "Part 2 Output: " << number * number_sequence[bingo_number_index-1] << "\n";

}

std::vector<unsigned int> parse_bingo_stream(std::string& line) {
    std::vector<unsigned int> numbers;
    std::stringstream stream(line);
    std::string output;

    while(getline(stream, output, ',')) {
        numbers.push_back(atoi(output.c_str()));
    }

    return numbers;
}

std::vector<Board> parse_bingo_boards(std::vector<std::string>& lines) {

    std::vector<Board> boards;

    unsigned int current_line = 2;

    while(current_line < lines.size()) {

        Board board;

        for(unsigned int i = 0; i < 5; i++) {
            board.grid[i][0].number = atoi(lines[current_line].substr(0, 2).c_str());
            board.grid[i][1].number = atoi(lines[current_line].substr(3, 5).c_str());
            board.grid[i][2].number = atoi(lines[current_line].substr(6, 8).c_str());
            board.grid[i][3].number = atoi(lines[current_line].substr(9, 11).c_str());
            board.grid[i][4].number = atoi(lines[current_line].substr(12, 14).c_str());

            current_line++;
        }
        current_line++;
        boards.push_back(board);
    }

    return boards;
}

int main() {

    std::ifstream input("input.txt");

    std::vector<std::string> lines;

    if(input.is_open()) {
        std::string data;
        while(std::getline(input, data)) {
            lines.push_back(data);
        }
        input.close();
    }

    std::vector<unsigned int> bingo_number_stream = parse_bingo_stream(lines[0]);
    std::vector<Board> bingo_boards = parse_bingo_boards(lines);

    part1(bingo_boards, bingo_number_stream);
    part2(bingo_boards, bingo_number_stream);

    return 0;
}