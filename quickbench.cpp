#include <vector>

static void SwapBigVecs(benchmark::State& state) {
  constexpr size_t vecSize = 1'000'000;
  std::vector<int> foo(vecSize, 0);
  std::vector<int> bar(vecSize, 1);

  // Code inside this loop is measured repeatedly
  for (auto _ : state) {
      foo.swap(bar);
      benchmark::DoNotOptimize(foo);
      benchmark::DoNotOptimize(bar);
  }
}
BENCHMARK(SwapBigVecs);

static void SwapSmallVecs(benchmark::State& state) {
  constexpr size_t vecSize = 5;
  std::vector<int> foo(vecSize, 0);
  std::vector<int> bar(vecSize, 1);

  for (auto _ : state) {
    foo.swap(bar);
    benchmark::DoNotOptimize(foo);
    benchmark::DoNotOptimize(bar);
  }
}
BENCHMARK(SwapSmallVecs);
