#include <iostream>
#include <string>
#include <cstdio>

using namespace std;

int main() {
  freopen("teleport.in", "r", stdin);
  freopen("teleport.out", "w", stdout);
  int a, b, x, y;
  cin >> a >> b >> x >> y;
  int dis1 = abs(b - a); // Calc straight
  int dis2 = abs(a - x) + abs(b - y); // Go to x, teleport to y then go to b;
  int dis3 = abs(a - y) + abs(b - x); // Go to y, teleport to x then go to b;
  cout << min(dis3, min(dis1, dis2));
}
