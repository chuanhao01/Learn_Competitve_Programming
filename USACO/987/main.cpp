#include <iostream>
#include <string>
#include <cstdio>
#include <sstream>
#include <vector>

using namespace std;

int main() {
  freopen("word.in", "r", stdin);
  freopen("word.out", "w", stdout);
  int n, k;
  cin >> n >> k;
  string words[n];
  int words_on_cur_line = 0;
  for (int i = 0; i < n; i++) {
    string word;
    cin >> word;
    words_on_cur_line += word.length();
    // We can add it to the cur line
    if (words_on_cur_line <= k) {
      if (i > 0) {
        cout << " ";
      }
      cout << word;
    } else {
      words_on_cur_line = word.length();
      cout << endl << word;
    }
  }
}
