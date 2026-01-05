// --feature-flag feature:feature-1, type: analysis, language: cpp, version: 1.0.0

#include <iostream>
#include <string>
#include <vector>

class Analyzer {
private:
    std::string name;
    std::vector<int> data;

public:
    Analyzer(const std::string& name) : name(name) {}

    void addData(int value) {
        data.push_back(value);
    }

    double calculateAverage() const {
        if (data.empty()) {
            return 0.0;
        }
        double sum = 0.0;
        for (int value : data) {
            sum += value;
        }
        return sum / data.size();
    }

    void printReport() const {
        std::cout << "Analyzer: " << name << std::endl;
        std::cout << "Average: " << calculateAverage() << std::endl;
    }
};
