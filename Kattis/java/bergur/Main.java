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
        int N = fio.nextInt();
        long[] nums = new long[N];
        for(int i=0;i<N; i++){
            nums[i] = fio.nextLong();
        }
        int j = N-1;
        long min = nums[j];
        long sum = min;
        j--;
        while(j >= 0){
            min = Math.min(min, nums[j]);
            sum += min;
            j--;
        }
        fio.println(sum);
        fio.close();
        return;
    }
}
