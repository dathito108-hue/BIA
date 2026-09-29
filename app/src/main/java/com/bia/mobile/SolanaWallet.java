package com.bia.mobile;

import android.app.Activity;
import android.content.Intent;
import android.net.Uri;
import java.util.concurrent.*;
import com.solana.mobilewalletadapter.clientlib.scenario.*;
import com.solana.mobilewalletadapter.clientlib.protocol.MobileWalletAdapterClient;

/** Wallet retains keys. Never clicks wallet approval UI or stores auth tokens on disk. */
final class SolanaWallet {
    interface Callback {void done(String address,byte[] signed,String error);}
    private String token="",owner="";
    private final ExecutorService worker=Executors.newSingleThreadExecutor();
    private volatile LocalAssociationScenario active;
    private volatile int generation;
    boolean busy(){return active!=null;}
    String owner(){return owner;}
    void request(Activity activity,SolanaWire.Transaction tx,boolean revoke,Callback callback){
        if(busy())return;
        final int g=++generation;final String auth=token,expected=owner;
        final LocalAssociationScenario scenario=new LocalAssociationScenario(60000);active=scenario;
        try {
            Intent intent=LocalAssociationIntentCreator.createAssociationIntent(null,scenario.getPort(),scenario.getSession());
            activity.startActivityForResult(intent,136);
            worker.execute(()->{
                String address="",newToken="",error=null;byte[] signed=null;
                try {
                    MobileWalletAdapterClient c=scenario.start().get(40,TimeUnit.SECONDS);
                    if(revoke){if(!auth.isEmpty())c.deauthorize(auth).get(60,TimeUnit.SECONDS);}
                    else {
                        MobileWalletAdapterClient.AuthorizationResult a=c.authorize(Uri.parse("https://github.com/dathito108-hue/BIA"),null,"BIA", "solana:mainnet",auth.isEmpty()?null:auth,null,null,null).get(60,TimeUnit.SECONDS);
                        address=SolanaWire.base58(a.accounts[0].publicKey);SolanaWire.address(address);newToken=a.authToken;
                        if(tx!=null){if(!address.equals(expected))throw new IllegalStateException("Ví/tài khoản đã thay đổi; tạo lại báo giá");if(g!=generation)throw new CancellationException();
                            signed=c.signTransactions(new byte[][]{tx.bytes.clone()}).get(60,TimeUnit.SECONDS).signedPayloads[0];tx.signed(signed,address);
                        }
                    }
                }catch(Exception e){error=e instanceof IllegalStateException?e.getMessage():"Ví từ chối, hết thời gian hoặc chưa hỗ trợ yêu cầu. Chưa gửi qua BIA.";}
                finally{scenario.close();}
                final String resultAddress=address,resultToken=newToken,resultError=error;final byte[] resultSigned=signed;
                activity.runOnUiThread(()->{if(g!=generation)return;active=null;if(resultError==null){owner=resultAddress;token=resultToken;}callback.done(owner,resultSigned,resultError);});
            });
        }catch(android.content.ActivityNotFoundException e){active=null;scenario.close();callback.done(owner,null,"Không tìm thấy ví hỗ trợ Mobile Wallet Adapter trên máy.");}
        catch(Exception e){active=null;scenario.close();callback.done(owner,null,"Không thể mở ví.");}
    }
    void cancel(){generation++;LocalAssociationScenario s=active;active=null;if(s!=null)NetworkCleanup.run(s::close);}
    void destroy(){cancel();token="";owner="";worker.shutdown();}
}
