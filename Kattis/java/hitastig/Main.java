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
        while (st == null || ! st.hasMoreElements()) {
            try { st = new StringTokenizer(br.readLine()); }
            catch (IOException  e) { e.printStackTrace(); }
        }
        return st.nextToken();
    }
    int nextInt() { return Integer.parseInt(next()); }
    long nextLong() { return Long.parseLong(next()); }
    double nextDouble() { return Double.parseDouble(next()); }
    String nextLine() {
        String str = "";
        try { str = br.readLine(); }
        catch (IOException e) { e.printStackTrace(); }
        return str;
    }
}

class Main {
    public static void main(String[] args) {
        FastIO fio = new FastIO();
        int n = fio.nextInt();
        long largestSoFar = Long.MIN_VALUE;
        long smallestSoFar = Long.MAX_VALUE;
        for (int i = 0; i < n; i++){
            long x = fio.nextLong();
            largestSoFar = Math.max(largestSoFar, x);
            smallestSoFar = Math.min(smallestSoFar, x);
        }
        fio.println(String.format("%d %d", largestSoFar, smallestSoFar));
        fio.close();
        return;
    }
}
