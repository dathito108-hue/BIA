package com.bia.mobile;
import okhttp3.*;
import java.util.concurrent.*;
/** TLS close can perform network I/O. Revoke state on UI; release sockets here. */
final class NetworkCleanup {
    private static final ExecutorService worker=Executors.newSingleThreadExecutor(r->new Thread(r,"bia-network-cleanup"));
    static void run(Runnable action){worker.execute(action);}
    static void close(OkHttpClient client,WebSocket socket){worker.execute(()->{
        try{if(socket!=null)socket.cancel();client.dispatcher().cancelAll();client.connectionPool().evictAll();}
        finally{client.dispatcher().executorService().shutdown();}
    });}
}
