#include <iostream>
#include <fstream>
#include <vector>
#include <queue>
#include <algorithm>

struct BoardMatrix {
    std::vector<std::vector<unsigned int>> grid;
    unsigned int width = 0;
    unsigned int height = 0;
};

struct Position {
    unsigned int x;
    unsigned int y;

    bool operator==(const Position& other) {
        return x == other.x && y == other.y;
    }
};

bool check_number(unsigned int x, unsigned int y, BoardMatrix& board) {
    return x >= 0 && x < board.width && y >= 0 && y < board.height;
}

bool is_lower(unsigned int x1, unsigned int y1, unsigned int x2, unsigned int y2, BoardMatrix& board) {
    if(check_number(x2, y2, board)) {
        if(board.grid[y1][x1] < board.grid[y2][x2]) {
            return true;
        } else {
            return false;
        }
    }
    return true;
}

void part1(BoardMatrix& matrix) {
    
    unsigned int score = 0;
    for(unsigned int x = 0; x < matrix.width; x++) {
        for(unsigned int y = 0; y < matrix.height; y++) {

            bool lower = false;

            if(is_lower(x, y, x+1, y, matrix) && is_lower(x, y, x-1, y, matrix) && is_lower(x, y, x, y+1, matrix) && is_lower(x, y, x, y-1, matrix)) {
                lower = true;
            }

            if(lower) {
                score += matrix.grid[y][x] + 1;
            }

        }
    }

    std::cout << "Part 1 Output: " << score << "\n";

}

bool check_basin_number(unsigned int x, unsigned int y, BoardMatrix& board) {
    return x >= 0 && x < board.width && y >= 0 && y < board.height && board.grid[y][x] != 9;
}

bool basin_number_already_exists(unsigned int x, unsigned int y, std::vector<Position>& already_exist) {
    Position pos = {x, y};
    return std::find(already_exist.begin(), already_exist.end(), pos) != already_exist.end();
}

unsigned int basin_search(unsigned int x, unsigned int y, BoardMatrix& board) {

    std::vector<Position> already_exist;
    std::queue<Position> queue;
    Position start;
    start.x = x;
    start.y = y;
    queue.push(start);
    already_exist.push_back(start);

    unsigned int total = 0;

    while(queue.size() > 0) {

        Position current = queue.front(); queue.pop();
        total++;

        if(check_basin_number(current.x + 1, current.y, board) && !basin_number_already_exists(current.x + 1, current.y, already_exist)) {
            Position new_pos;
            new_pos.x = current.x + 1;
            new_pos.y = current.y;
            queue.push(new_pos);
            already_exist.push_back(new_pos);
        }

        if(check_basin_number(current.x - 1, current.y, board) && !basin_number_already_exists(current.x - 1, current.y, already_exist)) {
            Position new_pos;
            new_pos.x = current.x - 1;
            new_pos.y = current.y;
            queue.push(new_pos);
            already_exist.push_back(new_pos);
        }

        if(check_basin_number(current.x, current.y + 1, board) && !basin_number_already_exists(current.x, current.y + 1, already_exist)) {
            Position new_pos;
            new_pos.x = current.x;
            new_pos.y = current.y + 1;
            queue.push(new_pos);
            already_exist.push_back(new_pos);
        }

        if(check_basin_number(current.x, current.y - 1, board) && !basin_number_already_exists(current.x, current.y - 1, already_exist)) {
            Position new_pos;
            new_pos.x = current.x;
            new_pos.y = current.y - 1;
            queue.push(new_pos);
            already_exist.push_back(new_pos);
        }

    }

    return total;

}

void part2(BoardMatrix& matrix) {

    std::priority_queue<unsigned int> basins;

    unsigned int score = 1; 
    for(unsigned int x = 0; x < matrix.width; x++) {
        for(unsigned int y = 0; y < matrix.height; y++) {

            bool lower = false;

            if(is_lower(x, y, x+1, y, matrix) && is_lower(x, y, x-1, y, matrix) && is_lower(x, y, x, y+1, matrix) && is_lower(x, y, x, y-1, matrix)) {
                lower = true;
            }

            if(lower) {
                basins.push(basin_search(x, y, matrix));
            }

        }
    }

    for(unsigned int i = 0; i < 3; i++) {
        score *= basins.top(); basins.pop();
    }

    std::cout << "Part 1 Output: " << score << "\n";
}

void print_matrix(BoardMatrix& matrix) {
    
    std::cout << "Width " << matrix.width << "\n";
    std::cout << "Height " << matrix.height << "\n";

    for(unsigned int x = 0; x < matrix.width; x++) {
        for(unsigned int y = 0; y < matrix.height; y++) {
            std::cout << matrix.grid[y][x];
        }
        std::cout << "\n";
    }

}

int main() {

    std::ifstream input("input.txt");
    BoardMatrix board;
    if(input.is_open()) {
        std::string data;
        unsigned int line = 0;
        while(std::getline(input, data)) {
            board.grid.push_back({});
            board.width = data.length();
            for(unsigned int i = 0; i < data.length(); i++) {
                unsigned int value = (unsigned int)data[i]-48;
                board.grid[line].push_back(value);
            }
            line++;
        }
        board.height = line;
        input.close();
    }

    //print_matrix(board);
    part1(board);
    part2(board);

    return 0;
}