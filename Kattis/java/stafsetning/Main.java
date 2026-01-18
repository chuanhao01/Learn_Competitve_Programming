import java.io.*;
import java.math.BigInteger;
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
        int k = fio.nextInt();
        BigInteger ns = BigInteger.ZERO;
        for (int i = 0; i < n; i++) {
            ns = ns.add(BigInteger.valueOf(fio.nextInt()));
        }
        if (m > k){
            fio.println(":(");
            fio.close();
            return;
        }
        BigInteger ans = ns.divide(BigInteger.valueOf(k/m));
        if (ns.mod(BigInteger.valueOf(k/m)).compareTo(BigInteger.ZERO) != 0 ){
            ans = ans.add(BigInteger.ONE);
        }
        fio.println(ans);
        fio.close();
        return;
    }
}
