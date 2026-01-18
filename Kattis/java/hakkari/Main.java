import java.io.*;
import java.util.*;

class FastIO extends PrintWriter {
    BufferedReader br;
    StringTokenizer st;

    public FastIO() {
        super(new BufferedOutputStream(System.out));
        br = new BufferedReader(new InputStreamReader(System.in));
    }

    String next() {
        while (st == null || !st.hasMoreElements()) {
            try {
                st = new StringTokenizer(br.readLine());
            } catch (IOException e) {
                e.printStackTrace();
            }
        }
        return st.nextToken();
    }

    int nextInt() {
        return Integer.parseInt(next());
    }

    long nextLong() {
        return Long.parseLong(next());
    }

    double nextDouble() {
        return Double.parseDouble(next());
    }

    String nextLine() {
        String str = "";
        try {
            str = br.readLine();
        } catch (IOException e) {
            e.printStackTrace();
        }
        return str;
    }
}

class Main {
    public static void main(String[] args) {
        FastIO fio = new FastIO();
        int n = fio.nextInt();
        int m = fio.nextInt();
        char[][] map = new char[n][m];
        ArrayList<int[]> ans = new ArrayList<int[]>();
        for (int i = 0; i < n; i++) {
            map[i] = fio.nextLine().toCharArray();
        }
        for (int y = 0; y < n; y++) {
            for (int x = 0; x < m; x++) {
                if (map[y][x] == '*'){
                    ans.add(new int[]{y, x});
                }
            }
        }
        // String l = fio.nextLine();
        fio.println(ans.size());
        for (int i=0; i<ans.size(); i++){
            fio.println(String.format("%d %d", ans.get(i)[0] + 1, ans.get(i)[1] + 1));
        }
        fio.close();
        return;
    }
}
