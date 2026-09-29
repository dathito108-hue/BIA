package com.bia.mobile;
import android.app.job.*;
import android.content.*;

/** OS-scheduled read-only recovery, including reboot. Android controls actual scheduling. */
public final class TradeRecoveryService extends JobService {
    volatile SolanaRpc rpc;
    static void schedule(Context c){JobScheduler scheduler=c.getSystemService(JobScheduler.class);scheduler.schedule(new JobInfo.Builder(137,new ComponentName(c,TradeRecoveryService.class)).setRequiredNetworkType(JobInfo.NETWORK_TYPE_ANY).setMinimumLatency(15000).setBackoffCriteria(30000,JobInfo.BACKOFF_POLICY_EXPONENTIAL).setPersisted(true).build());}
    public boolean onStartJob(JobParameters p){rpc=new SolanaRpc();final SolanaRpc r=rpc;new Thread(()->{boolean retry=true;try(TradeBook book=new TradeBook(this)){TradeReconcile.run(r,book);retry=!book.pending().isEmpty();}catch(Exception ignored){}finally{r.close();jobFinished(p,retry);}},"bia-reconcile-only").start();return true;}
    public boolean onStopJob(JobParameters p){if(rpc!=null)rpc.close();return true;}
}
