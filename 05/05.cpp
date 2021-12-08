#include <iostream>
#include <fstream>
#include <vector>
#include <sstream>
#include <cstring>
#include <map>
#include <tuple>
#include <cmath>

struct Line {
    int x1;
    int x2;
    int y1;
    int y2;
    int dirx;
    int diry;
};

int sign(float val) {
    if(val < 0.0) {
        return -1;
    } else {
        return 1;
    }
}

int get_dirx(Line& line) {

    float delta_x = (float)line.x2 - (float)line.x1;
    float delta_y = (float)line.y2 - (float)line.y1;

    if(delta_x != 0) {
        return sign(delta_x);
    } else {
        return 0;
    }

}

int get_diry(Line& line) {

    float delta_x = (float)line.x2 - (float)line.x1;
    float delta_y = (float)line.y2 - (float)line.y1;

    if(delta_y != 0) {
        return sign(delta_y);
    } else {
        return 0;
    }

}

void insert_to_grid(std::map<std::tuple<int, int>, int>& grid, int x, int y) {
    grid[std::make_tuple(x, y)]++;
}

int get_grid_value(std::map<std::tuple<int, int>, int>& grid, int x, int y) {
    return grid[std::make_tuple(x, y)];
}

void fill_grid_from_line(Line& line, std::map<std::tuple<int, int>, int>& grid, bool diagonal) {

    int start_x = line.x1;
    int start_y = line.y1;

    if(!diagonal) {
        if(start_x == line.x2 || start_y == line.y2) {
            while(!(start_x == line.x2 && start_y == line.y2)) {

                insert_to_grid(grid, start_x, start_y);
                start_x += line.dirx;
                start_y += line.diry;
            }
            insert_to_grid(grid, start_x, start_y);
        }
    } else {
        while(!(start_x == line.x2 && start_y == line.y2)) {

            insert_to_grid(grid, start_x, start_y);
            start_x += line.dirx;
            start_y += line.diry;

        }
        insert_to_grid(grid, start_x, start_y);
    }
}

void part1(std::vector<Line>& lines, std::map<std::tuple<int, int>, int> grid, int width, int height) {

    int score = 0;

    for(int i = 0; i < lines.size(); i++) {

        fill_grid_from_line(lines[i], grid, false);

    }
    
    for(int x = 0; x < width; x++) {
        for(int y = 0; y < height; y++) {
            
            if(get_grid_value(grid, x, y) > 1) {
                score++;
            }
        }  
    }

    std::cout << "Part 1 Output: " << score << "\n";

}

void part2(std::vector<Line>& lines, std::map<std::tuple<int, int>, int> grid, int width, int height) {

    int score = 0;

    for(int i = 0; i < lines.size(); i++) {

        fill_grid_from_line(lines[i], grid, true);

    }
    
    for(int x = 0; x < width; x++) {
        for(int y = 0; y < height; y++) {
            
            if(get_grid_value(grid, x, y) > 1) {
                score++;
            }
        }  
    }
    std::cout << "Part 2 Output: " << score << "\n";

}

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

int main() {

    std::ifstream input("input.txt");
    std::vector<Line> lines;

    int width = 0;
    int height = 0;

    if(input.is_open()) {
        std::string data;
        while(std::getline(input, data)) {

            std::vector<std::string> input_line = split_string(data, " -> ");
            std::vector<std::string> line_start = split_string(input_line[0], ",");
            std::vector<std::string> line_end = split_string(input_line[1], ",");

            Line line;
            line.x1 = atoi(line_start[0].c_str());
            line.y1 = atoi(line_start[1].c_str());
            line.x2 = atoi(line_end[0].c_str());
            line.y2 = atoi(line_end[1].c_str());

            line.dirx = get_dirx(line);
            line.diry = get_diry(line);

            if(line.x1 > width) {
                width = line.x1;
            }
            if(line.x2 > width) {
                width = line.x2;
            }

            if(line.y1 > height) {
                height = line.y1;
            }
            if(line.y2 > height) {
                height = line.y2;
            }

            lines.push_back(line);
            
        }
        input.close();
    }
    height++;
    width++;

    std::map<std::tuple<int, int>, int> grid; 

    part1(lines, grid, width, height);
    part2(lines, grid, width, height);

    return 0;
}