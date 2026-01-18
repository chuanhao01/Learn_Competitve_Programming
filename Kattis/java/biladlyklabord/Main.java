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
        String line = fio.nextLine();
        char[] charArr = line.toCharArray();
        // ArrayList<Character> ans = new ArrayList<Character>();
        String ans = "";
        int l = 0;
        int r = 1;
        while (l < charArr.length) {
            if (l == charArr.length - 1){
                ans += charArr[l];
                break;
            }
            if (charArr[l] != charArr[r]) {
                ans += charArr[l];
                l = r;
                r += 1;
            } else {
                r += 1;
                if (r >= charArr.length) {
                    ans += charArr[l];
                    break;
                }
            }
        }
        fio.println(ans);
        fio.close();
        return;
    }
}
