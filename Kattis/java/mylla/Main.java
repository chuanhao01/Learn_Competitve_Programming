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
        int n = fio.nextInt();
        char[] games = fio.nextLine().toCharArray();
        int a = 0;
        int h = 0;
        int aa = 0;
        int hh = 0;
        for (char c : games){
            if (c == 'A'){
                a += 1;
            } else {
                h += 1;
            }
            if (a > 2 || h > 2){
                if (a > h){
                    aa += 1;
                } else{
                    hh += 1;
                }
                a = 0;
                h = 0;
            }
        }
        // String l = fio.nextLine();
        if (hh < aa){
            fio.println("Hannes");
        } else {
            fio.println("Arnar");
        }
        fio.close();
        return;
    }
}
