package com.bia.mobile;

import java.util.*;
import java.math.*;
import java.security.MessageDigest;
import org.json.*;
import android.util.Base64;

/** Fail-closed decoder for legacy/v0 messages and known Jupiter V6 exact-in routes.
 * Route layouts: jup-ag/jupiter-cpi/idl.json (Apache-2.0); unknown variants are rejected.
 * Program execution still trusts the canonical Jupiter/SPL programs and the RPC provider.
 */
final class SolanaAudit {
    static final String TOKEN="TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
    static final String SYSTEM="11111111111111111111111111111111";
    static final String ATA="ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";
    static final String JUP="JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4";
    static final String COMPUTE="ComputeBudget111111111111111111111111111111";
    static final String ALT="AddressLookupTab1e1111111111111111111111111";
    static final BigInteger P=BigInteger.ONE.shiftLeft(255).subtract(BigInteger.valueOf(19));
    static final BigInteger D=BigInteger.valueOf(-121665).multiply(BigInteger.valueOf(121666).modInverse(P)).mod(P);
    static void require(boolean condition,String why){if(!condition)throw new IllegalStateException("Audit: "+why);}
    static BigInteger le(byte[] bytes,int offset,int size){require(size>0 && offset>=0 && offset+size<=bytes.length,"thiếu byte");byte[] b=new byte[size];for(int i=0;i<size;i++)b[size-1-i]=bytes[offset+i];return new BigInteger(1,b);}
    static byte[] hash(byte[]... chunks)throws Exception{MessageDigest d=MessageDigest.getInstance("SHA-256");for(byte[] b:chunks)d.update(b);return d.digest();}
    static boolean onCurve(byte[] b){byte[] a=b.clone();a[31]&=127;BigInteger y=le(a,0,32);if(y.compareTo(P)>=0)return false;BigInteger yy=y.multiply(y).mod(P),den=D.multiply(yy).add(BigInteger.ONE).mod(P);if(den.signum()==0)return false;BigInteger x2=yy.subtract(BigInteger.ONE).multiply(den.modInverse(P)).mod(P);return x2.signum()==0 || x2.modPow(P.subtract(BigInteger.ONE).shiftRight(1),P).equals(BigInteger.ONE);}
    static String ata(String owner,String mint)throws Exception{for(int bump=255;bump>=0;bump--){byte[] key=hash(SolanaWire.address(owner),SolanaWire.address(TOKEN),SolanaWire.address(mint),new byte[]{(byte)bump},SolanaWire.address(ATA),"ProgramDerivedAddress".getBytes(java.nio.charset.StandardCharsets.US_ASCII));if(!onCurve(key))return SolanaWire.base58(key);}throw new IllegalStateException("Không tạo được ATA");}
    static final class Reader {
        final byte[] bytes;int p;
        Reader(byte[] b){bytes=b;}
        int u8(){require(p<bytes.length,"giao dịch bị cắt");return bytes[p++]&255;}
        byte[] take(int n){require(n>=0 && n<=1232 && p+n<=bytes.length,"độ dài không hợp lệ");byte[] b=Arrays.copyOfRange(bytes,p,p+n);p+=n;return b;}
        BigInteger uint(int size){BigInteger n=le(bytes,p,size);p+=size;return n;}
        int compact(){int n=0;for(int i=0;i<3;i++){int b=u8();n|=(b&127)<<(7*i);if((b&128)==0){require(i==0 || b!=0,"shortvec không chuẩn");require(n<=1232,"shortvec quá lớn");return n;}}throw new IllegalStateException("shortvec quá dài");}
        void end(){require(p==bytes.length,"byte thừa chưa giải mã");}
    }
    static final class Ix {int program;int[] accounts;byte[] data;}
    static final class Lookup {String key;byte[] write,read;}
    static final class Message {
        final ArrayList<String> keys=new ArrayList<>();final ArrayList<Boolean> writable=new ArrayList<>();
        final ArrayList<Ix> instructions=new ArrayList<>();final ArrayList<Lookup> lookups=new ArrayList<>();
        Message(SolanaWire.Transaction tx){Reader r=new Reader(tx.message);boolean v0=(r.bytes[0]&128)!=0;if(v0)require(r.u8()==128,"phiên bản chưa hỗ trợ");require(r.u8()==1 && r.u8()==0,"phải có đúng một người ký");int readOnly=r.u8(),count=r.compact();require(count>=1 && count<=64 && readOnly<count,"header không hợp lệ");for(int i=0;i<count;i++){keys.add(SolanaWire.base58(r.take(32)));writable.add(i<count-readOnly);}r.take(32);int n=r.compact();require(n>0 && n<=32,"số chỉ thị không hợp lệ");for(int i=0;i<n;i++){Ix ix=new Ix();ix.program=r.u8();int k=r.compact();require(k<=64,"quá nhiều tài khoản");ix.accounts=new int[k];for(int a=0;a<k;a++)ix.accounts[a]=r.u8();ix.data=r.take(r.compact());instructions.add(ix);}if(v0){int size=r.compact();require(size<=8,"quá nhiều ALT");for(int i=0;i<size;i++){Lookup l=new Lookup();l.key=SolanaWire.base58(r.take(32));l.write=r.take(r.compact());l.read=r.take(r.compact());lookups.add(l);}}r.end();}
        void resolve(SolanaRpc rpc)throws Exception{
            ArrayList<String> writes=new ArrayList<>(),reads=new ArrayList<>();
            for(Lookup l:lookups){JSONObject result=(JSONObject)rpc.call("getAccountInfo",new JSONArray().put(l.key).put(SolanaRpc.object("encoding","base64","commitment","confirmed")));JSONObject a=result.getJSONObject("value");require(ALT.equals(a.getString("owner")),"sai chủ ALT");byte[] b=data(a);require(b.length>=56 && (b.length-56)%32==0 && le(b,0,4).intValue()==1,"ALT hỏng");require(le(b,4,8).equals(BigInteger.ONE.shiftLeft(64).subtract(BigInteger.ONE)),"ALT đang đóng");require(le(b,12,8).compareTo(BigInteger.valueOf(result.getJSONObject("context").getLong("slot")))<0,"ALT chưa đủ tuổi");for(byte idx:l.write){int p=56+(idx&255)*32;require(p+32<=b.length,"ALT index");writes.add(SolanaWire.base58(Arrays.copyOfRange(b,p,p+32)));}for(byte idx:l.read){int p=56+(idx&255)*32;require(p+32<=b.length,"ALT index");reads.add(SolanaWire.base58(Arrays.copyOfRange(b,p,p+32)));}}
            for(String s:writes){keys.add(s);writable.add(true);}for(String s:reads){keys.add(s);writable.add(false);}validate();
        }
        void validate(){require(keys.size()<=64 && new HashSet<>(keys).size()==keys.size(),"tài khoản trùng/quá nhiều");for(Ix i:instructions){require(i.program<keys.size() && !writable.get(i.program),"program index/quyền ghi");for(int k:i.accounts)require(k<keys.size(),"account index");}}
        String account(Ix ix,int n){require(n<ix.accounts.length,"thiếu tài khoản chỉ thị");return keys.get(ix.accounts[n]);}
    }
    static byte[] data(JSONObject a)throws Exception{return Base64.decode(a.getJSONArray("data").getString(0),Base64.DEFAULT);}
    static final int[] routeWidths={0,0,0,0,0,0,0,0,1,0,0,0,1,0,0,1,1,1,1,0,0,1,0,1,1,0,0,1,1,16,0,0,0,4,0,0,0,0,0};
    static boolean shared(byte[] d)throws Exception{byte[] disc=Arrays.copyOf(d,8);if(Arrays.equals(disc,Arrays.copyOf(hash("global:route".getBytes("UTF-8")),8)))return false;require(Arrays.equals(disc,Arrays.copyOf(hash("global:shared_accounts_route".getBytes("UTF-8")),8)),"loại route chưa được giải mã");return true;}
    static void routeArgs(byte[] data,SolanaOrder q,boolean shared){Reader r=new Reader(data);r.take(8);if(shared)r.u8();int n=r.uint(4).intValueExact();require(n>0 && n<=16,"route plan quá lớn");for(int i=0;i<n;i++){int variant=r.u8();require(variant<routeWidths.length,"AMM variant chưa hỗ trợ");int width=routeWidths[variant];if(width==1)require(r.u8()<=1,"bool/side không hợp lệ");else r.take(width);int percent=r.u8();require(percent>0 && percent<=100,"route percent");r.u8();r.u8();}
        require(r.uint(8).equals(q.inAmount),"lượng đầu vào trong byte không khớp");BigInteger quoted=r.uint(8);int bps=r.uint(2).intValue();int fee=r.u8();r.end();require(bps==q.slippage && fee==q.platformBps,"phí/slippage trong byte không khớp");require(quoted.multiply(BigInteger.valueOf(10000-bps)).divide(BigInteger.valueOf(10000)).compareTo(q.minOut)>=0,"min-out trong byte thấp hơn báo giá");}
    static void instructions(Message m,SolanaOrder q,String source,String dest)throws Exception{
        int swaps=0;BigInteger funded=BigInteger.ZERO;boolean limit=false,price=false;
        for(Ix ix:m.instructions){String program=m.keys.get(ix.program);Reader r=new Reader(ix.data);
            if(program.equals(COMPUTE)){require(ix.accounts.length==0,"compute accounts");int tag=r.u8();if(tag==2){require(!limit,"CU limit trùng");limit=true;BigInteger n=r.uint(4);require(n.signum()>0 && n.compareTo(BigInteger.valueOf(1400000))<=0,"CU limit");}else if(tag==3){require(!price,"CU price trùng");price=true;r.uint(8);}else require(false,"compute instruction chưa hỗ trợ");r.end();}
            else if(program.equals(SYSTEM)){require(r.uint(4).intValueExact()==2 && ix.accounts.length==2,"chỉ cho phép nạp WSOL qua transfer");BigInteger n=r.uint(8);r.end();require(q.input.equals(SolanaOrder.SOL) && m.account(ix,0).equals(q.owner) && m.account(ix,1).equals(source),"SOL gửi sai tài khoản");funded=funded.add(n);require(funded.compareTo(q.inAmount)<=0,"SOL nạp vượt lượng lệnh");}
            else if(program.equals(ATA)){require(ix.data.length==0 || (ix.data.length==1 && (ix.data[0]==0 || ix.data[0]==1)),"ATA chỉ thị chưa hỗ trợ");require(ix.accounts.length==6 || ix.accounts.length==7,"ATA accounts");String mint=m.account(ix,3);require((mint.equals(q.input)||mint.equals(q.output)) && m.account(ix,0).equals(q.owner) && m.account(ix,2).equals(q.owner) && m.account(ix,1).equals(ata(q.owner,mint)) && m.account(ix,4).equals(SYSTEM) && m.account(ix,5).equals(TOKEN),"ATA không thuộc ví/tài sản yêu cầu");}
            else if(program.equals(TOKEN)){int tag=r.u8();r.end();String wsol=ata(q.owner,SolanaOrder.SOL);if(tag==17)require(ix.accounts.length==1 && m.account(ix,0).equals(wsol),"SyncNative sai ATA");else if(tag==9)require(ix.accounts.length==3 && m.account(ix,0).equals(wsol) && m.account(ix,1).equals(q.owner) && m.account(ix,2).equals(q.owner),"CloseAccount sai chủ/người nhận");else require(false,"cấm approve/setAuthority/transfer bổ sung");}
            else if(program.equals(JUP)){require(++swaps==1,"nhiều swap trong một giao dịch");boolean s=shared(ix.data);routeArgs(ix.data,q,s);require(m.account(ix,0).equals(TOKEN) && m.account(ix,s?2:1).equals(q.owner) && m.account(ix,s?3:2).equals(source) && m.account(ix,s?6:3).equals(dest) && m.account(ix,s?8:5).equals(q.output) && m.account(ix,s?12:8).equals(JUP),"Jupiter sai nguồn/đích/chủ");if(s)require(m.account(ix,7).equals(q.input),"sai input mint");else require(m.account(ix,4).equals(JUP)||m.account(ix,4).equals(dest),"người nhận phụ ngoài ví");}
            else require(false,"program ngoài danh sách được hỗ trợ: "+program);
        }require(swaps==1,"thiếu swap đã giải mã");
    }
    static void token(JSONObject a,String owner,String mint,boolean zero)throws Exception{require(TOKEN.equals(a.getString("owner")) && !a.getBoolean("executable"),"token program không hợp lệ");byte[] b=data(a);require(b.length==165 && SolanaWire.base58(Arrays.copyOfRange(b,0,32)).equals(mint) && SolanaWire.base58(Arrays.copyOfRange(b,32,64)).equals(owner),"token sai mint/chủ/layout");require(b[108]==1 && le(b,72,4).signum()==0 && le(b,129,4).signum()==0,"token frozen/delegate/close authority");if(zero)require(le(b,64,8).signum()==0,"WSOL hiện có dư; chưa hỗ trợ đóng tài khoản đó");}
    static String verify(SolanaRpc rpc,SolanaOrder q)throws Exception{
        Message m=new Message(q.transaction);m.resolve(rpc);String source=ata(q.owner,q.input),dest=ata(q.owner,q.output);instructions(m,q,source,dest);
        JSONArray keys=new JSONArray();for(String k:m.keys)keys.put(k);
        JSONObject before=(JSONObject)rpc.call("getMultipleAccounts",new JSONArray().put(keys).put(SolanaRpc.object("encoding","base64","commitment","confirmed","dataSlice",SolanaRpc.object("offset",0,"length",165))));
        JSONArray values=before.getJSONArray("value");require(values.length()==m.keys.size(),"snapshot thiếu tài khoản");
        for(int i=0;i<values.length();i++){if(values.isNull(i))continue;JSONObject a=values.getJSONObject(i);if(TOKEN.equals(a.optString("owner"))){byte[] b=data(a);if(b.length==165 && SolanaWire.base58(Arrays.copyOfRange(b,32,64)).equals(q.owner) && m.writable.get(i)){String k=m.keys.get(i);require(k.equals(source)||k.equals(dest),"có quyền ghi token khác trong ví");token(a,q.owner,k.equals(source)?q.input:q.output,(k.equals(source)?q.input:q.output).equals(SolanaOrder.SOL));}}}
        int sourceIndex=m.keys.indexOf(source),destIndex=m.keys.indexOf(dest);require(sourceIndex>=0 && destIndex>=0,"thiếu ATA");for(int i:new int[]{sourceIndex,destIndex})if(!values.isNull(i))token(values.getJSONObject(i),q.owner,i==sourceIndex?q.input:q.output,(i==sourceIndex?q.input:q.output).equals(SolanaOrder.SOL));
        for(String mint:new String[]{q.input,q.output}){int i=m.keys.indexOf(mint);require(i>=0 && !values.isNull(i),"thiếu mint");JSONObject a=values.getJSONObject(i);byte[] b=data(a);require(TOKEN.equals(a.getString("owner")) && b.length==82 && b[45]==1 && (b[44]&255)==(mint.equals(SolanaOrder.SOL)?9:6),"mint không phải SPL chuẩn/đúng decimals");}
        JSONArray watch=new JSONArray().put(q.owner).put(source).put(dest);
        JSONObject sim=(JSONObject)rpc.call("simulateTransaction",new JSONArray().put(Base64.encodeToString(q.transaction.bytes,Base64.NO_WRAP)).put(SolanaRpc.object("encoding","base64","commitment","confirmed","minContextSlot",before.getJSONObject("context").getLong("slot"),"sigVerify",false,"replaceRecentBlockhash",false,"innerInstructions",true,"accounts",SolanaRpc.object("encoding","base64","addresses",watch))));
        JSONObject v=sim.getJSONObject("value");require(v.isNull("err"),"preflight lỗi");JSONArray after=v.getJSONArray("accounts");require(after.length()==3 && !after.isNull(0),"thiếu snapshot sau preflight");
        require(SYSTEM.equals(after.getJSONObject(0).getString("owner")) && !after.getJSONObject(0).getBoolean("executable"),"quyền ví bị đổi");
        BigInteger solBefore=new BigInteger(values.getJSONObject(0).get("lamports").toString()),solAfter=new BigInteger(after.getJSONObject(0).get("lamports").toString());
        int u=q.input.equals(SolanaOrder.USDC)?sourceIndex:destIndex,au=q.input.equals(SolanaOrder.USDC)?1:2;
        BigInteger usdcBefore=values.isNull(u)?BigInteger.ZERO:le(data(values.getJSONObject(u)),64,8);require(!after.isNull(au),"USDC account bị đóng");token(after.getJSONObject(au),q.owner,SolanaOrder.USDC,false);BigInteger usdcAfter=le(data(after.getJSONObject(au)),64,8);
        if(q.input.equals(SolanaOrder.SOL)){require(solBefore.subtract(solAfter).compareTo(q.inAmount.add(BigInteger.valueOf(q.feeBudget)))<=0 && usdcAfter.subtract(usdcBefore).compareTo(q.minOut)>=0,"delta tài sản vượt giới hạn/min-out");}
        else require(usdcBefore.subtract(usdcAfter).compareTo(q.inAmount)<=0 && solAfter.subtract(solBefore).compareTo(q.minOut.subtract(BigInteger.valueOf(q.feeBudget)))>=0,"delta tài sản vượt giới hạn/min-out");
        return "Audit legacy/v0 + ALT + route + tài khoản + delta preflight: đạt (phụ thuộc chương trình/RPC)";
    }
}
