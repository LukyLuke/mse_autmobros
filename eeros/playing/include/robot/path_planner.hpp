#pragma once

#include <vector>
#include <tuple>

class PathPlanner {
public:
	PathPlanner() {
		path.push_back(std::make_tuple(0, 0, 0));
	}

	~PathPlanner() {}

	void add(uint32_t x, uint32_t y, uint32_t turn_radius) {
		path.push_back(std::make_tuple(x, y, turn_radius));
	}

private:
	std::vector<std::tuple<uint32_t, uint32_t, uint32_t>> path;
};
