#include <cstdlib>
#include <iostream>
#include <vector>
#include <cassert>

int main(int argc, char **argv) {
  constexpr size_t defaultVecSize = 1'000'000;
  constexpr size_t nSwaps = 10'000 - 1;

  const size_t vecSize = [&] {
    if (argc >= 2) {
      return static_cast<size_t>(atoi(argv[1]));
    } else {
      return defaultVecSize;
    }
  }();

  std::cout << "vec size: " << vecSize << "\n";
  std::cout << "nswaps: " << nSwaps << "\n";

  std::vector<int> foo(vecSize, 0);
  std::vector<int> bar(vecSize, 1);

  for (int i = 0; i < nSwaps; i++) {
    foo.swap(bar);
  }

  assert(foo[0] == 1);
  assert(foo[vecSize - 1] == 1);
  assert(bar[0] == 0);
  assert(bar[vecSize - 1] == 0);

  return 0;
}
